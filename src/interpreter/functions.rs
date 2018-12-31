use crate::ast::expressions::function;

use crate::interpreter::{self, environment, types};
use crate::utils;

impl interpreter::Eval for function::Closure {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let parameters: Vec<String> = self.
            params.
            iter().
            map(
                |exp| {
                    if let types::Type::String(string) = exp.eval(env) {
                        string
                    } else {
                        self.runtime_error(format!("Function arguments contains not a string, but {:?}", exp));
                    }
                }).
            collect();

        types::Type::Function {
            id: env.borrow_mut().next_global_id(),
            parameters,
            varargs: self.varargs,
            body: self.body.clone(),
            env: env.clone()
        }
    }
}