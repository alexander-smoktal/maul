
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use utils::Shared;
use interpreter::types;

#[derive(Debug)]
pub struct Environment {
    global_id_counter: Rc<RefCell<u64>>,
    data: HashMap<String, Rc<RefCell<types::Type>>>,
    parent: Option<Shared<Environment>>
}

impl Environment {
    pub fn new(parent: Option<Shared<Environment>>) -> Self {
        Environment {
            global_id_counter: Rc::new(RefCell::new(0)),
            data: HashMap::new(),
            parent
        }
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

impl Deref for Environment {
    type Target = HashMap<String, Rc<RefCell<types::Type>>>;

    fn deref(&self) -> &HashMap<String, Rc<RefCell<types::Type>>> {
        &self.data
    }
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Environment {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self.data) == other.to_string()
    }
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Shared<Environment> {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self.borrow().data) == other.to_string()
    }
}
