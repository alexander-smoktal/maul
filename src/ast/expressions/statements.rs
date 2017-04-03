use super::Expression;

#[derive(Debug)]
pub enum Statement {
    Break,
}

impl Expression for Statement {}