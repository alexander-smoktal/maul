use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;

use crate::ast::expressions;
use crate::interpreter::environment;
use crate::utils;

pub enum Type {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    /// Reference to an existing value
    Reference(Rc<RefCell<Type>>),
    Vector(VecDeque<Type>),
    Table {
        /// For comparison
        id: u64,
        map: HashMap<Type, Rc<RefCell<Type>>>,
        metatable: HashMap<String, Type>,
        border: usize,
    },
    Function {
        /// For comparison
        id: u64,
        parameters: Vec<String>,
        varargs: bool,
        body: Rc<Box<dyn expressions::Expression>>,
        // XXX: Capture only vars function needs?
        env: utils::Shared<environment::Environment>,
    },
}

impl Type {
    pub fn call(&self, _arguments: Vec<&Type>) -> Type {
        unimplemented!();
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Type::Nil => false,
            Type::Boolean(false) => false,
            _ => true,
        }
    }

    /// Check if type is nil. We often have special cases for nils
    pub fn is_nil(&self) -> bool {
        match self {
            Type::Nil => true,
            Type::Reference(typeref) => typeref.borrow().is_nil(),
            _ => false,
        }
    }

    /// Create reference to an object or clone reference
    pub fn into_reference(self) -> Rc<RefCell<Self>> {
        match self {
            Type::Reference(typeref) => typeref,
            _ => Rc::new(RefCell::new(self)),
        }
    }
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Type {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self) == *other
    }
}

impl ::std::cmp::PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Boolean(left), Type::Boolean(right)) => left == right,
            (Type::Number(left), Type::Number(right)) => left == right,
            (Type::String(left), Type::String(right)) => left == right,
            (Type::Reference(left), right) => right.eq(left.borrow().deref()),
            (left, Type::Reference(right)) => left.eq(right.borrow().deref()),
            (Type::Vector(left), Type::Vector(right)) => left == right,
            (Type::Table { id: left, .. }, Type::Table { id: right, .. }) => left == right,
            (Type::Function { id: left, .. }, Type::Function { id: right, .. }) => left == right,
            _ => false,
        }
    }
}

impl ::std::cmp::Eq for Type {}

impl ::std::hash::Hash for Type {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Type::Nil => 1.hash(state),
            Type::Boolean(value) => value.hash(state),
            Type::Number(value) => value.to_string().hash(state),
            Type::String(value) => value.hash(state),
            Type::Reference(value) => value.borrow().hash(state),
            Type::Vector(vec) => vec.hash(state),
            Type::Table { id, .. } => id.hash(state),
            Type::Function { id, .. } => id.hash(state),
        }
    }
}

/// Display to correctly infor user about runtime errors
impl ::std::fmt::Display for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Type::Function { id, .. } => write!(f, "function ({:x})", id),
            Type::Table { id, .. } => write!(f, "table ({:x})", id),
            Type::Reference(value) => value.borrow().fmt(f),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Debug, which breaks closured env circular dependency
impl ::std::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Type::Nil => write!(f, "Nil"),
            Type::Boolean(value) => write!(f, "Boolean({:?})", value),
            Type::Number(value) => write!(f, "Number({:?})", value),
            Type::String(value) => write!(f, "String({:?})", value),
            Type::Reference(value) => write!(f, "Reference({:?})", value),
            Type::Vector(vec) => write!(f, "Vector({:?})", vec),
            Type::Table {
                id,
                map,
                metatable,
                border,
            } => write!(
                f,
                "Table {{ id: {}, map: {:?}, metatable: {:?}, border: {} }}",
                id, map, metatable, border
            ),
            Type::Function {
                id,
                parameters,
                varargs,
                body,
                env,
            } => write!(
                f,
                "Function {{ id: {:?}, parameters: {:?}, varargs: {:?}, body: {:?}, env: {:?} }}",
                id,
                parameters,
                varargs,
                body,
                env.borrow().id()
            ),
        }
    }
}

/// Macro to use for pattern maching types with respect to type referencing
#[macro_export]
macro_rules! match_type {
    (($($typ:expr),+), $($pat:pat => $result:expr),+) => {{
        let typs = ($(if let $crate::interpreter::types::Type::Reference(value) = $typ { unsafe { &*value.as_ptr() } } else { $typ }),+);

        match typs {$(
            $pat => $result
        ), +}}
    };
    ($typ:expr, $($pat:pat => $result:expr),+) => {{
        let typ = if let $crate::interpreter::types::Type::Reference(value) = $typ { unsafe { &*value.as_ptr() } } else { $typ };

        #[allow(clippy::single_match)]
        match typ {$(
            $pat => $result
        ), +}}
    };
}

impl ::std::convert::AsRef<bool> for Type {
    fn as_ref(&self) -> &bool {
        match_type!(&self,
            Type::Boolean(val) => val,
            _ => panic!("Cannot convert lua value {} to a boolean", self)
        )
    }
}

impl ::std::convert::AsRef<f64> for Type {
    fn as_ref(&self) -> &f64 {
        match_type!(&self,
            Type::Number(val) => val,
            _ => panic!("Cannot convert lua value {} to a number", self)
        )
    }
}

impl ::std::convert::AsRef<String> for Type {
    fn as_ref(&self) -> &String {
        match_type!(&self,
            Type::String(val) => val,
            _ => panic!("Cannot convert lua value {} to a string", self)
        )
    }
}

impl ::std::convert::AsRef<VecDeque<Type>> for Type {
    fn as_ref(&self) -> &VecDeque<Type> {
        match_type!(&self,
            Type::Vector(val) => val,
            _ => panic!("Cannot convert lua value {} to a deque", self)
        )
    }
}

impl ::std::convert::AsRef<HashMap<Type, Rc<RefCell<Type>>>> for Type {
    fn as_ref(&self) -> &HashMap<Type, Rc<RefCell<Type>>> {
        match_type!(&self,
            Type::Table { map, .. } => map,
            _ => panic!("Cannot convert lua value {} to a hashmap", self)
        )
    }
}