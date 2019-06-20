use crate::ast::expressions::{self, blocks};
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
            if let environment::BreakFlag::Break(true) = env.borrow().break_flag() {
                return types::Type::Nil;
            }
        }

        if let Some(ref retstat) = self.retstat {
            let return_value = retstat.eval(env);

            if !env
                .borrow_mut()
                .break_execution(environment::BreakFlag::Return(Some(return_value)))
            {
                self.runtime_error("Unexpected return statement. Not a function".to_string())
            }
        }

        types::Type::Nil
    }
}

// pub struct DoBlock(pub Box<expressions::Expression>);
impl interpreter::Eval for blocks::DoBlock {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let local_env =
            environment::Environment::new(Some(env.clone()), environment::BreakFlag::Break(false));

        self.0.eval(&mut utils::Shared::new(local_env));
        types::Type::Nil
    }
}

// TODO: Do we need another wrapper for local?
// pub struct Local(Box<expressions::Expression>);
impl interpreter::Eval for blocks::Local {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        self.0.eval(env)
    }
}

// pub struct WhileBlock {
//     pub condition: Box<expressions::Expression>,
//     pub block: Box<expressions::Expression>,
// }
impl interpreter::Eval for blocks::WhileBlock {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let mut local_env = utils::Shared::new(environment::Environment::new(
            Some(env.clone()),
            environment::BreakFlag::Break(false),
        ));

        while self.condition.eval(env).as_bool() {
            self.block.eval(&mut local_env);
            // Check if broken
            let env_borrow = local_env.borrow();
            if let environment::BreakFlag::Break(true) = *env_borrow.break_flag() {
                break;
            }
        }
        types::Type::Nil
    }
}

// pub struct RepeatBlock {
//     pub block: Box<expressions::Expression>,
//     pub condition: Box<expressions::Expression>,
// }
impl interpreter::Eval for blocks::RepeatBlock {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let mut local_env = utils::Shared::new(environment::Environment::new(
            Some(env.clone()),
            environment::BreakFlag::Break(false),
        ));

        loop {
            self.block.eval(&mut local_env);

            // Check if broken
            if let environment::BreakFlag::Break(true) = local_env.borrow().break_flag() {
                break;
            }

            if self.condition.eval(env).as_bool() {
                break;
            }
        }

        types::Type::Nil
    }
}

// pub struct IfCondition {
//     pub condition: Box<expressions::Expression>,
//     pub block: Box<expressions::Expression>,
// }
impl interpreter::Eval for blocks::IfCondition {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        if self.condition.eval(env).as_bool() {
            self.block.eval(env);
            return types::Type::Boolean(true);
        }

        types::Type::Nil
    }
}

// pub struct IfBlock {
//     pub conditions: VecDeque<Box<expressions::Expression>>,
//     pub else_block: Option<Box<expressions::Expression>>,
// }
impl interpreter::Eval for blocks::IfBlock {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        for condition in &self.conditions {
            if condition.eval(env).as_bool() {
                return types::Type::Nil;
            }
        }

        match &self.else_block {
            Some(block) => block.eval(env),
            _ => types::Type::Nil,
        }
    }
}

// pub struct NumericalForBlock {
//     pub init: Box<expressions::Expression>,
//     pub limit: Box<expressions::Expression>,
//     pub step: Option<Box<expressions::Expression>>,
//     pub block: Box<expressions::Expression>,
// }
impl interpreter::Eval for blocks::NumericalForBlock {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        // Statement initialization
        let eval_name = self.var_name.eval(env);
        let var_name = match_type!(&eval_name,
            types::Type::String(value) => value.clone(),
            _ => self.runtime_error(format!("{:?} cannot be used as `for` statement variable name", self.var_name))
        );

        let mut get_num = |exp: &Box<expressions::Expression>, value_type| -> f64 {
            let evaluated = exp.eval(env);
            match_type!(&evaluated,
                types::Type::Number(value) => *value,
                _ => self.runtime_error(format!("{:?} cannot be used as `for` statement {} value", exp, value_type))
            )
        };

        let init_num = get_num(&self.init_value, "initial");
        let limit_num = get_num(&self.limit, "limit");

        let step_num = if let Some(step) = &self.step {
            get_num(step, "step")
        } else {
            1f64
        };

        // Creating local env and assigning initial value
        let mut local_env = utils::Shared::new(environment::Environment::new(
            Some(env.clone()),
            environment::BreakFlag::Break(false),
        ));

        let mut i = init_num;
        let counter_ref = local_env
                .borrow_mut()
                .add_variable(var_name.clone(), types::Type::Number(i));

        while (i - limit_num).abs() > std::f64::EPSILON {
            counter_ref.replace(types::Type::Number(i));

            self.block.eval(&mut local_env);

            // Check if broken
            if let environment::BreakFlag::Break(true) = local_env.borrow().break_flag() {
                break;
            }

            i += step_num;
        }

        types::Type::Nil
    }
}
