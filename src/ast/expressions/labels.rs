use super::expression;

#[derive(Debug)]
pub struct Label(pub Box<expression::Expression>);
impl expression::Expression for Label {}

#[derive(Debug)]
pub struct Goto(pub Box<expression::Expression>);
impl expression::Expression for Goto {}
