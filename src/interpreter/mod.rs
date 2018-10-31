pub mod environment;
pub mod operators;
pub mod primitives;
pub mod tables;
pub mod types;
pub mod variables;
pub mod blocks;

pub trait Eval {
    fn eval<'this>(&'this self, _env: &'this mut environment::Environment) -> types::Type {
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
