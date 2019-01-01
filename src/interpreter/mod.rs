#[macro_use]
pub mod types;
pub mod blocks;
pub mod environment;
pub mod expression;
pub mod functions;
pub mod operators;
pub mod primitives;
pub mod statements;
pub mod tables;
pub mod variables;

use crate::ast::expressions;
use crate::utils;

pub trait Eval: std::fmt::Debug {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        println!("{:?} `eval` unimplemented", self);
        unimplemented!();
    }

    #[cfg(test)]
    fn runtime_error(&self, error: String) -> ! {
        panic!("Runtime error: {}", error);
    }

    #[cfg(not(test))]
    fn runtime_error(&self, error: String) -> ! {
        println!("Runtime error: {}", error);
        ::std::process::exit(1)
    }
}

impl Eval for expressions::Terminal {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::Nil
    }
}
