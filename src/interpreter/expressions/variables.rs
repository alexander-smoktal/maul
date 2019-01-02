use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;

use crate::ast::expressions::{self, variables};
use crate::interpreter::{self, environment, types};
use crate::utils;
use std::collections::VecDeque;

const DEBUG: bool = false;

// pub struct Id(pub String);
impl interpreter::Eval for variables::Id {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        // Let's check if we can retrieve value from cache
        if let Some(cached_value) = self.get_cached(&*env.borrow()) {
            if DEBUG {
                println!("Returning cached value {:?}: {:?}", self.id, cached_value);
            }
            return cached_value
        };

        let find_result = env.borrow_mut().get(&self.id).clone();
        let var_reference = if let Some(refcell) = find_result {
            refcell
        } else {
            let new_entry = Rc::new(RefCell::new(types::Type::Nil));
            env.borrow_mut()
                .add_variable(self.id.clone(), types::Type::Reference(new_entry.clone()));
            new_entry
        };

        // Save value into cache
        self.set_cached(&*env.borrow(), &var_reference);
        if DEBUG {
            println!("Saving cached value {:?}: {:?}", self.id, var_reference);
        }
        types::Type::Reference(var_reference)
    }
}

/// Function to evaluate vars and expressions and properly append result into a target
fn eval_expression(
    expressions: &VecDeque<Box<expressions::Expression>>,
    env: &mut utils::Shared<environment::Environment>,
) -> VecDeque<types::Type> {
    let mut result = VecDeque::new();

    for exp in expressions {
        match exp.eval(env) {
            types::Type::Vector(vec) => result.extend(vec.into_iter()),
            value => result.push_back(value),
        }
    }

    result
}

// pub struct Assignment {
//     pub varlist: VecDeque<Box<expressions::Expression>>,
//     pub explist: VecDeque<Box<expressions::Expression>>,
// }
impl interpreter::Eval for variables::Assignment {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        // TODO: Rework to use deque and pop values
        let mut var_typevec = eval_expression(&self.varlist, env);
        let mut exp_typevec = eval_expression(&self.explist, env);

        while !var_typevec.is_empty() {
            let key = var_typevec.pop_front().unwrap();

            let value = if exp_typevec.is_empty() {
                types::Type::Nil
            } else {
                exp_typevec.pop_front().unwrap()
            };

            if DEBUG {
                println!("Executing assignment {:?} = {:?}", key, value)
            }

            // We can only add variable by name, this should be an Id or change reference. Everything else is an error
            match key {
                types::Type::String(var_id) => {
                    env.borrow_mut().add_variable(var_id, value);
                },
                types::Type::Reference(reference) => {
                    reference.replace(value);
                }
                _ => self.runtime_error(format!("Can't use '{}' as a lvalue", key)),
            }
        }

        types::Type::Nil
    }
}
