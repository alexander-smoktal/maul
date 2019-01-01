use crate::ast::expressions::expression;

use crate::interpreter::{self, environment, types};
use crate::utils;

impl interpreter::Eval for expression::Expressions {
    // struct Expressions(VecDeque<Box<expressions::Expression>>);
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let mut result = types::Type::Nil;

        for exp in &self.0 {
            result = exp.eval(env)
        }

        result
    }
}