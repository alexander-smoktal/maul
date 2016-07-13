pub mod lexer;
pub mod parser;

use self::parser::expressions;

pub struct AST {
    expressions: expressions::Expressions
}