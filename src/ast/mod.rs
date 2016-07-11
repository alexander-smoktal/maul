pub mod lexer;
pub mod parser;

use std::vec::Vec;

use self::parser::Expression;

pub struct AST {
    expressions: Vec<Expression>
}