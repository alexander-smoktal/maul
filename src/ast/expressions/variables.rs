use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[derive(Debug, Clone)]
pub struct Id(String);
impl expression::Expression for Id {
    fn clone(&self) -> Box<expression::Expression> {
        Box::new(Id(self.0.clone()))
    }
}

#[derive(Debug)]
pub struct Assignment(pub Box<expression::Expression>, pub Box<expression::Expression>);
impl expression::Expression for Assignment {}
