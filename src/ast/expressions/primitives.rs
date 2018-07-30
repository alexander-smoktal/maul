use super::expression;
use super::utils;
use ast::lexer::tokens;
use ast::parser;

use std::string::String as StdString;

#[derive(Debug, Clone)]
pub struct String(pub StdString);
impl expression::Expression for String {}

impl String {
    pub fn rule(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
        if let Some(&tokens::Token { token: tokens::TokenType::String(ref string), ..}) = parser.next() {
            utils::some_expression(String(string.clone()))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number(pub f64);
impl expression::Expression for Number {}

impl Number {
    pub fn rule(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
        if let Some(&tokens::Token { token: tokens::TokenType::Number(ref number), ..}) = parser.next() {
            utils::some_expression(Number(number.clone()))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);
impl expression::Expression for Boolean {}

impl Boolean {
    pub fn rule(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
        match parser.next() {
            Some(&tokens::Token { token: tokens::TokenType::Keyword(tokens::Keyword::TRUE), ..}) => {
                utils::some_expression(Boolean(true))
            },
            Some(&tokens::Token { token: tokens::TokenType::Keyword(tokens::Keyword::FALSE), ..}) => {
                utils::some_expression(Boolean(false))
            },
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nil;
impl expression::Expression for Nil {}

impl Nil {
    pub fn rule(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
        if let Some(&tokens::Token { token: tokens::TokenType::Keyword(tokens::Keyword::NIL), ..}) = parser.next() {
            utils::some_expression(Nil {})
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Noop;
impl expression::Expression for Noop {}