use std::vec::Vec;

use super::*;
use ast::lexer;
use ast::lexer::tokens;

#[allow(dead_code, unused_variables)]
pub struct Function {
    name: String,
    args: Vec<id::Id>,
    body: Expressions,
}

impl Expression for Function {}

impl Function {
    fn parse_func_name(lexer: &mut lexer::Lexer) -> String {
        let mut result = String::new();

        if let tokens::TokenType::Id(ref name) = lexer[0].token {
            result += name;
        } else {
            panic!("Expected function name, got {:?}", lexer[0].token)
        }

        // TODO: Add complex names
        result
    }

    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Self {
        let func = Function {
            name: Function::parse_func_name(lexer),
            args: Vec::new(),
            body: vec![Box::new(Stub {})],
        };

        func
    }
}