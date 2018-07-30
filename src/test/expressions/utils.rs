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

pub fn parse_string(source_code: &str) -> Option<Box<expression::Expression>> {
    expression::Expressions::rule(&mut parser::Parser::new(source_code.to_string()))
}
