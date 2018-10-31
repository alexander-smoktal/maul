use ast::expressions::blocks;
use interpreter::{self, environment, types};

/* pub struct Block {
    statements: VecDeque<Box<expressions::Expression>>,
    retstat: Option<Box<expressions::Expression>>,
}*/
impl interpreter::Eval for blocks::Block {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        for ref statement in &self.statements {
            statement.eval(env);

            // Check if broken
            if env.brake_flag() != &environment::BreakFlag::None {
                return types::Type::Nil
            }
        }

        if let Some(ref retstat) = self.retstat {
            let return_value = retstat.eval(env);

            if !env.brake_execution(environment::BreakFlag::Return(Some(return_value))) {
                self.runtime_error("Unexpected return statement. Not a function".to_string())
            }
        }

        types::Type::Nil
    }
}

// pub struct DoBlock(pub Box<expressions::Expression>);
impl interpreter::Eval for blocks::DoBlock {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let local_env = environment::Environment::new(Some(env));

        self.0.eval(&mut local_env)
    }
}