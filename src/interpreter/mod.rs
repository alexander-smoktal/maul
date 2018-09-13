pub mod types;
pub mod environment;
pub mod primitives;

pub trait Eval {
    fn eval(&self, _env: &mut environment::Environment) -> types::Type {
        unimplemented!();
    }
}