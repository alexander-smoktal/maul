use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Indexing {
    object: Box<expressions::Expression>,
    index:  Box<expressions::Expression>
}

impl expressions::Expression for Indexing {}

impl Indexing {
    /// .Name indexing
    pub fn new_object(stack: &mut stack::Stack) {
        // Remove dot from stack
        let (name, _dot) = stack_unpack!(stack, single, single);
        stack.push_single(name);
        Indexing::new(stack)
    }

    /// ["String"] indexing
    pub fn new_table(stack: &mut stack::Stack) {
        // Remove brackets from stack
        let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
        stack.push_single(expression);
        Indexing::new(stack)
    }

    /// Createx indexing for its prefix
    fn new(stack: &mut stack::Stack) {
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

