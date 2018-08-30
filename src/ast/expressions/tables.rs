use super::*;

#[derive(Debug)]
pub struct Indexing (pub Box<expression::Expression>);
impl expression::Expression for Indexing {}

#[derive(Debug)]
pub struct TableConstructor(pub Vec<Box<expression::Expression>>);
impl expression::Expression for TableConstructor {}

