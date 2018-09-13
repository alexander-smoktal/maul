use std::collections::HashMap;
use std::ops::Deref;

use interpreter::types;

#[derive(Debug)]
pub struct Environment<'e> {
    data: HashMap<String, types::Type>,
    parent: Option<&'e Environment<'e>>
}

impl<'e> Environment<'e> {
    pub fn new(parent: Option<&'e Environment<'e>>) -> Self {
        Environment {
            data: HashMap::new(),
            parent
        }
    }

    pub fn parent(&'e self) -> Option<&'e Environment<'e>> {
        self.parent
    }
}

impl<'e> Deref for Environment<'e> {
    type Target = HashMap<String, types::Type>;

    fn deref(&self) -> &HashMap<String, types::Type> {
        &self.data
    }
}

#[cfg(test)]
impl<'e> ::std::cmp::PartialEq<&'static str> for Environment<'e> {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self.data) == other.to_string()
    }
}
