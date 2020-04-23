pub use crate::ast::expressions::Expression;
use crate::ast::parser;
use crate::ast::stack;

#[macro_export]
macro_rules! sexp {
    ($e: expr) => {
        Some(Box::new($e) as Box<dyn Expression>)
    };
}

#[macro_export]
macro_rules! exp {
    ($e: expr) => {
        Box::new($e) as Box<dyn Expression>
    };
}

#[allow(dead_code)]
pub fn parse_string<F>(source_code: &str, func: F) -> stack::Stack
where
    F: Fn(&mut parser::Parser, &mut stack::Stack) -> bool,
{
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::default();

    func(&mut parser, &mut stack);

    assert!(
        parser.peek().is_none(),
        "Parser contains tokens after parsing"
    );

    stack
}
