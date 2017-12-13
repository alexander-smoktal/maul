use std::fmt::Debug;

use super::*;
use ast::lexer;

pub trait Expression: Debug {
}

pub fn from_lexer(lexer: &mut lexer::Lexer) -> Option<Box<Expression>> {
    blocks::parse_block(lexer).ok().map(|e| e)
}