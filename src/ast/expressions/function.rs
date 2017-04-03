use std::vec::Vec;

use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Function {
    params: Vec<variables::Id>,
    body: Box<Expression>,
}

impl Expression for Function {}

impl Function {
    fn parse_func_args(lexer: &mut lexer::Lexer) -> Vec<variables::Id> {
        let mut result = vec![];

        if lexer.get(0).token == tokens::TokenType::Keyword(tokens::Keyword::LBRACE) {
            lexer.skip(1);

            while let tokens::TokenType::Id(name) = lexer.get(0).token.clone() {
                result.push(name);
                lexer.skip(1);
            }

            if lexer.get(0).token != tokens::TokenType::Keyword(tokens::Keyword::RBRACE) {
                error::Error::new(&lexer.get(0))
                    .complain("Expected ')' at the end of parameter list, got:");
            } else {
                lexer.skip(1);
            }
        } else {
            error::Error::new(&lexer.get(0)).complain("Expected function parameters start, got:");
        }

        result
    }

    fn parse_method_name(lexer: &mut lexer::Lexer) -> Option<variables::Id> {
        if lexer.get(0).token == tokens::TokenType::Keyword(tokens::Keyword::SEMICOLONS) {
            lexer.skip(1);

            if let tokens::TokenType::Id(name) = lexer.get(0).token.clone() {
                lexer.skip(1);
                Some(name)
            } else {
                error::Error::new(&lexer.get(0)).complain("Expected method name, got:");
                unreachable!()
            }
        } else {
            None
        }
    }

    pub fn from_lexer(lexer: &mut lexer::Lexer) -> variables::Assignment {
        // First parse function name as variable
        let mut function_name = match variables::Assignment::parse_varname(lexer) {
            Ok(varname) => varname,
            Err(e) => {
                e.complain("Failed to parse function name, expected id, got:");
                unreachable!()
            }
        };

        // Then parse method name if method
        let mut params = vec![];
        if let Some(method_name) = Self::parse_method_name(lexer) {
            function_name.push(method_name);
            params.push("self".to_owned())
        }

        // Parse function arguments
        params.append(&mut Self::parse_func_args(lexer));

        let func = Function {
            params: params,
            body: Box::new(Stub {}),
        };

        // Return assignment, because of function is a sugar for var
        variables::Assignment::new(function_name, Box::new(func))
    }
}