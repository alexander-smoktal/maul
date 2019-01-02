use crate::ast::expressions::statements;
use crate::interpreter::{self, environment, types};
use crate::utils;

impl interpreter::Eval for statements::Statement {
    // pub enum Statement {
    //     Break,
    //     Ellipsis,
    //     Return(Option<Box<expressions::Expression>>),
    // }
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        match &self {
            statements::Statement::Break => {
                env.borrow_mut()
                    .break_execution(environment::BreakFlag::Break(true));
                types::Type::Nil
            }
            statements::Statement::Ellipsis => unimplemented!("Hey, do you wanna some varargs?"),
            statements::Statement::Return(retval) => {
                // Block already handles return mechanism, so we just return value
                if let Some(expression) = retval {
                    expression.eval(env)
                } else {
                    types::Type::Nil
                }
            }
        }
    }
}
