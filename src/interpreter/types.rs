use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Table(HashMap<String, Type>, usize),
    Vector(Vec<Type>),
    Function(u32)
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Type {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self) == other.to_string()
    }
}