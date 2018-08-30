use super::*;
use ast::parser;

#[derive(Debug, Clone)]
pub struct Id(String);
impl expression::Expression for Id {}

impl Id {
    pub fn name(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
        parser.peek().and_then(|token| token.id()).map(|string| Box::new(Id(string)) as Box<expression::Expression>)
    }
}

#[derive(Debug)]
pub struct Varlist {
    head: Box<expression::Expression>,
    tail: Option<Box<expression::Expression>>
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
