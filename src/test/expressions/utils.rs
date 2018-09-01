pub use ast::expressions::Expression;
use ast::parser;
use ast::stack;

#[macro_export]
macro_rules! sexp {
    ($e: expr) => (Some(Box::new($e) as Box<Expression>));
}

#[macro_export]
macro_rules! exp {
    ($e: expr) => (Box::new($e) as Box<Expression>);
}

#[allow(dead_code)]
pub fn parse_string<F>(source_code: &str, func: F) -> stack::Stack 
    where F: Fn(&mut parser::Parser, &mut stack::Stack) -> bool {
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::new();

    func(&mut parser, &mut stack);
    stack
}
