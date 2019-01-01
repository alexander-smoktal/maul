use crate::ast::expressions::primitives;
use crate::interpreter::{self, environment, types};
use crate::utils;

impl interpreter::Eval for primitives::Nil {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::Nil
    }
}

impl interpreter::Eval for primitives::Boolean {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::Boolean(self.0)
    }
}

impl interpreter::Eval for primitives::Number {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::Number(self.0)
    }
}

impl interpreter::Eval for primitives::String {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::String(self.0.clone())
    }
}
