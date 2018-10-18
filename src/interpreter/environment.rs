use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use interpreter::types;

#[derive(Debug)]
pub struct Environment<'e> {
    global_id_counter: Rc<RefCell<u64>>,
    data: HashMap<String, Rc<RefCell<types::Type>>>,
    parent: Option<&'e Environment<'e>>
}

impl<'e> Environment<'e> {
    pub fn new(parent: Option<&'e Environment<'e>>) -> Self {
        Environment {
            global_id_counter: Rc::new(RefCell::new(0)),
            data: HashMap::new(),
            parent
        }
    }

    pub fn parent(&'e self) -> Option<&'e Environment<'e>> {
        self.parent
    }

    pub fn next_global_id(&mut self) -> u64 {
        let value = *self.global_id_counter.borrow();
        *self.global_id_counter.borrow_mut() = value;
        value
    }

    pub fn get(&mut self, varname: &String) -> Option<Rc<RefCell<types::Type>>> {
        self.data.get(varname).cloned()
    }

    pub fn add_variable(&mut self, id: String, value: types::Type) {
        match value {
            types::Type::Reference(value) => self.data.insert(id, value.clone()),
            value => self.data.insert(id, Rc::new(RefCell::new(value))),
        };
    }
}

impl<'e> Deref for Environment<'e> {
    type Target = HashMap<String, Rc<RefCell<types::Type>>>;

    fn deref(&self) -> &HashMap<String, Rc<RefCell<types::Type>>> {
        &self.data
    }
}

#[cfg(test)]
impl<'e> ::std::cmp::PartialEq<&'static str> for Environment<'e> {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self.data) == other.to_string()
    }
}