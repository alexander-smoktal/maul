use ast::expressions;

#[derive(Debug)]
pub struct DoBlock(pub Box<expressions::Expression>);
impl expressions::Expression for DoBlock {}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Box<expressions::Expression>,
    pub block: Box<expressions::Expression>,
}
impl expressions::Expression for WhileBlock {}

#[derive(Debug)]
pub struct RepeatBlock {
    pub block: Box<expressions::Expression>,
    pub condition: Box<expressions::Expression>,
}
impl expressions::Expression for RepeatBlock {}

// We could make typedef for 'while' and 'repeat', but can't implement trait for type
#[derive(Debug)]
pub struct Condition {
    pub condition: Box<expressions::Expression>,
    pub block: Box<expressions::Expression>,
}

#[derive(Debug)]
pub struct IfBlock {
    pub conditions: Vec<Condition>,
    pub elseblock: Option<Box<expressions::Expression>>,
}
impl expressions::Expression for IfBlock {}

// block ::= {stat} [retstat]


// do block end


// while exp do block end


// repeat block until exp

// if exp then block {elseif exp then block} [else block] end |
