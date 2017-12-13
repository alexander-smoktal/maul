use super::*;

#[derive(Debug)]
pub struct Binop(pub tokens::Keyword, pub Box<expression::Expression>, pub Box<expression::Expression>);
impl expression::Expression for Binop {}

#[derive(Debug)]
pub struct Unop(pub tokens::Keyword, pub Box<expression::Expression>);
impl expression::Expression for Unop {}