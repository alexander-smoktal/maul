use std::collections::HashMap;

use ast::expressions;

#[derive(Debug)]
pub enum Type {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Table {
        map: HashMap<String, Type>,
        metatable: HashMap<String, Type>,
        border: usize
    },
    Vector(Vec<Type>),
    Function {
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
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Type {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self) == other.to_string()
    }
}