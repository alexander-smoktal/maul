pub mod environment;
pub mod operators;
pub mod primitives;
pub mod tables;
pub mod types;
pub mod variables;
pub mod blocks;

use utils;

pub trait Eval {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
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
