use std::rc::Rc;
// use std::cell::RefCell;

use std::collections::VecDeque;
use ast::expressions::{ self, variables };
use interpreter::{self, environment, types};

// pub struct Id(pub String);
impl interpreter::Eval for variables::Id {
    fn eval(&self, _env: &mut environment::Environment) -> types::Type {
        types::Type::Id(self.0.clone())
    }
}

/// Function to evaluate vars and expressions and properly append result into a target
fn fill_out_typevec(exp: &Box<expressions::Expression>,
                    env: &mut environment::Environment,
                    typevec: &mut VecDeque<types::Type>) {
    match exp.eval(env) {
        types::Type::Vector(vec) => typevec.extend(vec.into_iter()),
        value => typevec.push_back(value)
    }

}

// pub struct Assignment {
//     pub varlist: VecDeque<Box<expressions::Expression>>,
//     pub explist: VecDeque<Box<expressions::Expression>>,
// }
impl interpreter::Eval for variables::Assignment {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        // TODO: Rework to use deque and pop values
        let mut var_typevec: VecDeque<types::Type> = VecDeque::new();
        let mut exp_typevec: VecDeque<types::Type> = VecDeque::new();

        self.varlist.iter().for_each(|exp| { fill_out_typevec(exp, env, &mut var_typevec) });
        self.explist.iter().for_each(|exp| { fill_out_typevec(exp, env, &mut exp_typevec) });

        while !var_typevec.is_empty() {
            let key = Rc::try_unwrap(var_typevec.pop_front().unwrap().as_ref()).unwrap().into_inner();

            let value = if exp_typevec.is_empty() {
                types::Type::Nil
            } else {
                exp_typevec.pop_front().unwrap()
            };

            // We can only add variable by name, this should be an Id or change reference. Everything else is an error
            match key {
                types::Type::Id(var_id) => env.add_variable(var_id, value),
                types::Type::Reference(reference) => { reference.replace(value); },
                _ => self.runtime_error(format!("Can't use '{}' as a lvalue", key))
            }
        }

        types::Type::Nil
    }
}
