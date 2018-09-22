// use std::rc::Rc;
// use std::cell::RefCell;

use ast::expressions::{ self, variables };
use interpreter::{self, environment, types};

// pub struct Id(pub String);
impl interpreter::Eval for variables::Id {
    fn eval(&self, _env: &mut environment::Environment) -> types::Type {
        types::Type::Id(self.0.clone())
    }
}

fn fill_out_typevec(exp: &Box<expressions::Expression>,
                    env: &mut environment::Environment,
                    typevec: &mut Vec<types::Type>) {
    match exp.eval(env) {
        types::Type::Vector(vec) => typevec.extend(vec.into_iter()),
        value => typevec.push(value)
    }
    
}

// pub struct Assignment {
//     pub varlist: VecDeque<Box<expressions::Expression>>,
//     pub explist: VecDeque<Box<expressions::Expression>>,
// }
impl interpreter::Eval for variables::Assignment {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let mut var_typevec: Vec<types::Type> = vec![];
        let mut exp_typevec: Vec<types::Type> = vec![];

        self.varlist.iter().for_each(|exp| { fill_out_typevec(exp, env, &mut var_typevec) });
        self.explist.iter().for_each(|exp| { fill_out_typevec(exp, env, &mut exp_typevec) });

        for _var in 0..var_typevec.len() {
            
        }

        types::Type::Nil
    }
}
