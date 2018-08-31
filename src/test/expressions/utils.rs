use ast::expressions::*;
use ast::parser;

#[macro_export]
macro_rules! sexp {
    ($e: expr) => (Some(Box::new($e) as Box<expression::Expression>));
}

#[macro_export]
macro_rules! exp {
    ($e: expr) => (Box::new($e) as Box<expression::Expression>);
}

#[allow(dead_code)]
pub fn parse_string(source_code: &str) -> Option<Box<expression::Expression>> {
    parser::rules::exp(&mut parser::Parser::new(source_code.to_string()))
}
