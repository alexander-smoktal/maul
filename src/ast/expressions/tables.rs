use ast::expressions;

#[derive(Debug)]
pub struct Indexing (pub Box<expressions::Expression>);
impl expressions::Expression for Indexing {}

#[derive(Debug)]
pub struct TableConstructor(pub Vec<Box<expressions::Expression>>);
impl expressions::Expression for TableConstructor {}

