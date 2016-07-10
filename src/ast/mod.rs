pub mod lexer_module {
    pub mod lexer;
    pub mod tokens;
    mod tests;
}

pub mod parser_module {
    pub mod parser;
    mod tests;
}

use std::vec::Vec;

use super::ast::parser_module::parser::Expression;

pub struct AST {
    expressions: Vec<Expression>
}