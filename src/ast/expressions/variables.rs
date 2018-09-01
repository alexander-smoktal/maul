use super::*;
use ast::stack;
use ast::parser;
use ast::lexer::tokens;

#[derive(Debug, Clone)]
pub struct Id(pub String);
impl expression::Expression for Id {}

impl Id {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token { token: tokens::TokenType::Id(string), ..}) = parser.peek().cloned() {
            parser.shift();
            stack.push_single(Box::new(Id(string)));
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Varlist {
    pub head: Box<expression::Expression>,
    pub tail: Option<Box<expression::Expression>>
}
impl expression::Expression for Varlist {}

impl Varlist {
    pub fn new(head: Box<expression::Expression>,
        _comma: Option<Box<expression::Expression>>,
        tail: Option<Box<expression::Expression>>) -> Option<Box<expression::Expression>> {
        utils::some_expression(Varlist {
            head,
            tail
        })
    }
}

#[derive(Debug)]
pub struct Assignment(pub Box<expression::Expression>, pub Box<expression::Expression>);
impl expression::Expression for Assignment {}
