use super::*;

#[derive(Debug)]
pub struct DoBlock(pub Box<expression::Expression>);
impl expression::Expression for DoBlock {}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Box<expression::Expression>,
    pub block: Box<expression::Expression>,
}
impl expression::Expression for WhileBlock {}

#[derive(Debug)]
pub struct RepeatBlock {
    pub block: Box<expression::Expression>,
    pub condition: Box<expression::Expression>,
}
impl expression::Expression for RepeatBlock {}

// We could make typedef for 'while' and 'repeat', but can't implement trait for type
#[derive(Debug)]
pub struct Condition {
    pub condition: Box<expression::Expression>,
    pub block: Box<expression::Expression>,
}

#[derive(Debug)]
pub struct IfBlock {
    pub conditions: Vec<Condition>,
    pub elseblock: Option<Box<expression::Expression>>,
}
impl expression::Expression for IfBlock {}

// block ::= {stat} [retstat]


// do block end


// while exp do block end


// repeat block until exp

// if exp then block {elseif exp then block} [else block] end |
