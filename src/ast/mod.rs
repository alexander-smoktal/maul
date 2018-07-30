pub mod lexer;
#[macro_use]
pub mod parser;
pub mod expressions;

use std::fmt::{Debug, Formatter, Error};

pub struct AST {
    expressions: Box<expressions::expression::Expression>,
}

impl AST {
    pub fn new(source_code: String) -> Self {
        AST {
            expressions: expressions::expression::from_parser(&mut parser::Parser::new(source_code)).unwrap_or(Box::new(expressions::common::Noop))
        }
    }
}

impl Debug for AST {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "{:?}", self.expressions)
    }
}
