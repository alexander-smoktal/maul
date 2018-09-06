use std::collections::VecDeque;

use ast::expressions;
use ast::rules;
use ast::stack;

#[derive(Debug)]
pub struct Indexing {
    object: Box<expressions::Expression>,
    index: Box<expressions::Expression>,
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
        rules::remove_enclosing_brackets(stack);
        Indexing::new(stack)
    }

    /// Createx indexing for its prefix
    fn new(stack: &mut stack::Stack) {
        let (index, object) = stack_unpack!(stack, single, single);

        stack.push_single(Box::new(Indexing { object, index }));
    }
}

#[derive(Debug)]
pub struct TableField {
    key: Option<Box<expressions::Expression>>,
    value: Box<expressions::Expression>
}

impl expressions::Expression for TableField {}

impl TableField {
    // terminal!(Keyword::LSBRACKET), exp, terminal!(Keyword::RSBRACKET), terminal!(Keyword::EQUAL), exp
    pub fn new_table_index(stack: &mut stack::Stack) {
        let (value, _assign, _rb, key, _lb) = stack_unpack!(stack, single, single, single, single, single);

        stack.push_single(Box::new(TableField {
            key: Some(key),
            value
        }))
    }

    // variables::Id::rule, terminal!(Keyword::EQUAL), exp
    pub fn new_object_index(stack: &mut stack::Stack) {
        let (value, _assign, key) = stack_unpack!(stack, single, single, single);

        stack.push_single(Box::new(TableField {
            key: Some(key),
            value
        }))
    }

    // exp
    pub fn new_value(stack: &mut stack::Stack) {
        let value = stack.pop_single();

        stack.push_single(Box::new(TableField {
            key: None,
            value
        }))
    }

    /// Sequence of fields. We either first field or consequential
    pub fn new_list_name(stack: &mut stack::Stack) {
        let field = stack.pop_single();

        match stack.peek() {
            // If we already had fields before
            stack::Element::Repetition(_) => {
                let mut fieldlist = stack.pop_repetition();
                fieldlist.push_back(field);
                stack.push_repetition(fieldlist)
            }
            // First field
            _ => {
                stack.push_repetition(VecDeque::from(vec![field]))
            }
        }
    }
}

#[derive(Debug)]
pub struct Table(VecDeque<Box<expressions::Expression>>);
impl expressions::Expression for Table {}

impl Table {
    // tableconstructor ::= ‘{’ [fieldlist] ‘}’
    pub fn new(stack: &mut stack::Stack) {
        let _rbr = stack.pop_single();

        // If had some fieldlist
        if let stack::Element::Repetition(_) = stack.peek() {
            let (fieldlist, _lbr) = stack_unpack!(stack, repetition, single);

            stack.push_single(Box::new(Table(fieldlist)))
        } else {
            let _lbr = stack.pop_single();
            stack.push_single(Box::new(Table(VecDeque::new())))
        }
    }
}
