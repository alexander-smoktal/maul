use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Indexing {
    object: Box<expressions::Expression>,
    index:  Box<expressions::Expression>
}

impl expressions::Expression for Indexing {}

impl Indexing {
    /// Createx indexing for its prefix
    pub fn new(stack: &mut stack::Stack) {
        let (index, object) = stack_unpack!(stack, single, single);

        stack.push_single(Box::new(
            Indexing {
                object,
                index
           }));
    }
}

#[derive(Debug)]
pub struct TableConstructor(pub Vec<Box<expressions::Expression>>);
impl expressions::Expression for TableConstructor {}

