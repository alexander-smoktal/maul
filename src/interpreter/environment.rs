use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::interpreter::types;
use crate::utils::Shared;

const DEBUG: bool = false;

macro_rules! debug_env {
    ($($output: expr),+) => {
        if DEBUG {
            println!($($output,)+)
        }
    };
}

/// Flag, which marks block env as interruptible and contains break type.
/// Non-interruptable blocks should check if flag is `None`, because break statements can cross block boundaries.
///   - `return` interruption may cross breakable and non-breakable blocks until reached funtion env;
///   - `break` interruption may cross non-breakable blocks until reacheck breakable block;
///   - `goto Name` interruption may interrupt any block execution to check if we need to jump.
#[derive(Debug, PartialEq, Eq)]
pub enum BreakFlag {
    /// Execution cannot be interrupted except cases with labels. I.e. `do` blocks.
    /// Such block boundaries can be crossed with any interraption toward the parrent env, untiv we found suitable block
    None,
    /// Breakable block: while, repeat, for
    /// Break interruption may cross unbrakable blocks boundaries (general purpose blocks)
    /// until we spot breakable block.
    Break(bool),
    /// Function block
    /// Return interruption may cross breakable blocks and non-breakable as well until innermost function block spotted
    Return(Option<types::Type>),
    /// Goto may interrupt execution of any block. Should be checked by all blocks and handled gracefuly
    /// Label name and old break flag value
    Goto(String, Box<BreakFlag>),
}

/// Environment structure. Each block starts new environment, settings current as a parent
#[derive(Debug)]
pub struct Environment {
    /// Global counter across all environments to set object ID's
    global_id_counter: Rc<RefCell<u64>>,
    /// Variables registry from current environment
    data: HashMap<String, Rc<RefCell<types::Type>>>,
    /// Parent environment. Used to reference to a calling code
    parent: Option<Shared<Environment>>,
    /// Block execution break flag. See BreakFlag documentation
    break_flag: BreakFlag,
    /// Environment self ID
    id: u64
}

impl Environment {
    pub fn new(parent: Option<Shared<Environment>>, break_flag: BreakFlag) -> Self {
        let global_id_counter = if let Some(ref parent) = parent {
                parent.borrow_mut().global_id_counter.clone()
            } else {
                Rc::new(RefCell::new(0))
            };
        let id = *global_id_counter.borrow();
        *global_id_counter.borrow_mut() += 1;

        Environment {
            global_id_counter,
            data: HashMap::new(),
            parent,
            break_flag,
            id,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    /// Global ID's to use for objects
    pub fn next_global_id(&mut self) -> u64 {
        let value = *self.global_id_counter.borrow();
        *self.global_id_counter.borrow_mut() += 1;
        value
    }

    /// Get variable value(reference). If current env doesn't contain the varable, checks in parent environments
    pub fn get(&mut self, varname: &str) -> Option<Rc<RefCell<types::Type>>> {
        if let Some(value) = self.data.get(varname) {
            debug_env!("Env found variable {}, which is {:?}", varname, value);
            Some(value.clone())
        } else {
            match self.parent {
                Some(ref mut parent) => {
                    debug_env!("Env didn't found variable {:?}, asking parent env", varname);
                    parent.borrow_mut().get(varname)
                }
                _ => {
                    debug_env!("Env didn't found variable {:?}, returning nil", varname);
                    None
                }
            }
        }
    }

    /// Add variable with `id` with `value` to the current environment
    pub fn add_variable(&mut self, id: String, value: types::Type) -> Rc<RefCell<types::Type>> {
        let reference = match value {
            types::Type::Reference(value) => value.clone(),
            value => Rc::new(RefCell::new(value)),
        };

        self.data.insert(id, reference.clone());
        reference
    }

    /// Set environment break flag. We cross blocks borders toward the topmost env and brake all blocks on the way
    pub fn break_execution(&mut self, flag: BreakFlag) -> bool {
        match flag {
            BreakFlag::None => false,
            BreakFlag::Break(_) => {
                match self.break_flag {
                    // Break crosses non-breakable block boundaries, and we also break current block execution
                    BreakFlag::None => {
                        if let Some(ref mut parent) = self.parent {
                            self.break_flag = BreakFlag::Break(true);
                            parent.borrow_mut().break_execution(flag)
                        } else {
                            // Didn't find breakable block
                            false
                        }
                    }
                    BreakFlag::Break(_) => {
                        self.break_flag = flag;
                        true
                    }
                    _ => false,
                }
            }
            BreakFlag::Return(_) => {
                match self.break_flag {
                    // Return crosses non-breakable block boundaries, and we also break current block execution
                    BreakFlag::None => {
                        if let Some(ref mut parent) = self.parent {
                            self.break_flag = BreakFlag::Break(true);
                            parent.borrow_mut().break_execution(flag)
                        } else {
                            // Didn't find returnable block
                            false
                        }
                    }
                    // Return crosses breakable block boundaries, and we also break current block execution
                    BreakFlag::Break(_) => {
                        if let Some(ref mut parent) = self.parent {
                            self.break_flag = BreakFlag::Break(true);
                            parent.borrow_mut().break_execution(flag)
                        } else {
                            // Didn't find returnable block
                            false
                        }
                    }
                    BreakFlag::Return(_) => {
                        self.break_flag = flag;
                        true
                    }
                    _ => false,
                }
            }
            // This doesn't look nice. We could fill old value from the inside. Will be todo
            BreakFlag::Goto(_, _) => {
                self.break_flag = flag;
                true
            }
        }
    }

    pub fn break_flag(&self) -> &BreakFlag {
        &self.break_flag
    }

    // Destroy environment and in case it contains return value, return it
    pub fn retval(&mut self) -> types::Type {
        if let BreakFlag::Return(ref mut some_ret) = self.break_flag {
            if let Some(retval) = some_ret.take() {
                return retval;
            }
        }

        types::Type::Nil
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();

        let mut result = "{".to_string();
        for key in keys.into_iter() {
            result += format!("{:?}: {:?}, ", key, self.data.get(key).unwrap()).as_str();
        }

        if !self.data.is_empty() {
            result.pop();
            result.pop();
        }

        result += "}";

        write!(formatter, "{}", result)
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
        format!("{}", self) == *other
    }
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Shared<Environment> {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{}", self.borrow()) == *other
    }
}
