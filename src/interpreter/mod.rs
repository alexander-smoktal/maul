#[macro_use]
pub mod types;
pub mod environment;
pub mod expressions;
pub mod cache;
pub mod native;

use crate::ast;
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

impl Eval for ast::expressions::Terminal {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::Nil
    }
}
