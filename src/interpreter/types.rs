use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ast::expressions;

#[derive(Debug)]
pub enum Type {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Reference(Rc<RefCell<Type>>),
    Vector(Vec<Type>),
    Table {
        /// For comparison
        id: u64,
        map: HashMap<Type, Rc<RefCell<Type>>>,
        metatable: HashMap<String, Type>,
        border: usize
    },
    Function {
        /// For comparison
        id: u64,
        parameters: Vec<String>,
        varargs: bool,
        body: Box<expressions::Expression>
    }
}

impl Type {
    pub fn call(&self, _arguments: Vec<&Type>) -> Type {
        unimplemented!();
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Type::Nil => false,
            Type::Boolean(false) => false,
            _ => true
        }
    }

    /// Unwrap value from reference
    pub fn unwrap(&self) -> &Self {
        if let Type::Reference(value) = self {
            unsafe {
                &(*value.as_ptr())
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Type {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self) == other.to_string()
    }
}

impl ::std::cmp::PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
       match (self, other) {
            (Type::Nil, Type::Nil) => true,
            (Type::Boolean(left), Type::Boolean(right)) => left == right,
            (Type::Number(left), Type::Number(right)) => left == right,
            (Type::String(left), Type::String(right)) => left == right,
            (Type::Reference(left), Type::Reference(right)) => left == right,
            (Type::Vector(left), Type::Vector(right)) => left == right,
            (Type::Table{ id: left, .. }, Type::Table{ id: right, .. }) => left == right,
            (Type::Function{ id: left, .. }, Type::Function{ id: right, .. }) => left == right,
            _ => false
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
            Type::Table{ id, .. } => id.hash(state),
            Type::Function{ id, .. } => id.hash(state)
        }
    }
}