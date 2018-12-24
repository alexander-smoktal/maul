use crate::ast::expressions::blocks;
use crate::interpreter::{self, environment, types};
use crate::utils;

/* pub struct Block {
    statements: VecDeque<Box<expressions::Expression>>,
    retstat: Option<Box<expressions::Expression>>,
}*/
impl interpreter::Eval for blocks::Block {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        for ref statement in &self.statements {
            statement.eval(env);

            // Check if broken
            if env.borrow().brake_flag() != &environment::BreakFlag::None {
                return types::Type::Nil
            }
        }

        if let Some(ref retstat) = self.retstat {
            let return_value = retstat.eval(env);

            if !env.borrow_mut().brake_execution(environment::BreakFlag::Return(Some(return_value))) {
                self.runtime_error("Unexpected return statement. Not a function".to_string())
            }
        }

        types::Type::Nil
    }
}

// pub struct DoBlock(pub Box<expressions::Expression>);
impl interpreter::Eval for blocks::DoBlock {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let local_env = environment::Environment::new(Some(env.clone()));

        self.0.eval(&mut utils::Shared::new(local_env))
    }
}