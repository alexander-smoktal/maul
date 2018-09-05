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

use std::collections::VecDeque;
use std::fmt::{Debug, Error, Formatter};

pub struct AST {
    expressions: VecDeque<Box<expressions::Expression>>,
}

impl AST {
    pub fn new(source_code: String) -> Self {
        let mut parser = parser::Parser::new(source_code);
        let mut stack = stack::Stack::new();

        rules::exp(&mut parser, &mut stack);

        AST {
            expressions: stack.pop_repetition(),
        }
    }
}

impl Debug for AST {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "{:?}", self.expressions)
    }
}
