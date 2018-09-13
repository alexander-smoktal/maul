use ast::expressions;
use ast::lexer::tokens;
use ast::parser;
use ast::stack;

use std::string::String as StdString;

#[derive(Debug, Clone)]
pub struct Nil;
impl expressions::Expression for Nil {}

impl Nil {
    make_keyword_rule![rule, (tokens::Keyword::NIL, Nil)];
}

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);
impl expressions::Expression for Boolean {}

impl Boolean {
    make_keyword_rule![
        rule,
        (tokens::Keyword::TRUE, Boolean(true)),
        (tokens::Keyword::FALSE, Boolean(false))
    ];
}

#[derive(Debug, Clone)]
pub struct Number(pub f64);
impl expressions::Expression for Number {}

impl Number {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token {
            token: tokens::TokenType::Number(number),
            ..
        }) = parser.peek().cloned()
        {
            parser.shift();
            stack.push_single(Box::new(Number(number)));
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct String(pub StdString);
impl expressions::Expression for String {}

impl String {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token {
            token: tokens::TokenType::String(string),
            ..
        }) = parser.peek().cloned()
        {
            parser.shift();
            stack.push_single(Box::new(String(string)));
            true
        } else {
            false
        }
    }
}
