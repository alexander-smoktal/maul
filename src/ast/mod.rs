#[macro_export]
macro_rules! debug_parser {
    ($($params: expr), +) => {
        if DEBUG {
            println!($($params,) +);
        }
    };
}

pub mod lexer;
#[macro_use]
pub mod stack;
#[macro_use]
pub mod grammar_macros;
pub mod expressions;
pub mod parser;
pub mod rules;

use std::fmt::{Debug, Error, Formatter};

pub struct AST {
    top_expression: Box<expressions::Expression>,
}

impl AST {
    pub fn new(source_code: String) -> Self {
        let mut parser = parser::Parser::new(source_code);
        let mut stack = stack::Stack::new();

        rules::chunk(&mut parser, &mut stack);

        AST {
            top_expression: stack.pop_single(),
        }
    }

    pub fn eval(&self) {
        let mut env = crate::utils::Shared::new(crate::interpreter::environment::Environment::new(None));

        self.top_expression.eval(&mut env);
    }
}

impl Debug for AST {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "{:?}", self.top_expression)
    }
}
