use std::rc::Rc;
use std::cell::RefCell;

use super::types;

/// Evaluation cache. Know how to store reference for a particular execution context: environment of table.
/// Called to speedup execution of block cycles
#[derive(Debug, Clone)]
pub struct Cache {
    /// Context environment or table id
    env_id: u64,
    /// Cached value if set
    cached_value: Option<Rc<RefCell<types::Type>>>
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            env_id: 0,
            cached_value: None
        }
    }

    pub fn get(&self, env_id: u64) -> Option<types::Type> {
        if env_id == self.env_id {
            if let Some(value) = &self.cached_value {
                return Some(types::Type::Reference(value.clone()))
            }
        }

        None
    }

    pub fn set(&mut self, env_id: u64, value: &Rc<RefCell<types::Type>>) {
        self.env_id = env_id;
        self.cached_value = Some(value.clone());
    }
}