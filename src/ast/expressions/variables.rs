use ast::expressions;
use ast::lexer::tokens;
use ast::parser;
use ast::stack;

#[derive(Debug, Clone)]
pub struct Id(pub String);
impl expressions::Expression for Id {}

impl Id {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token {
            token: tokens::TokenType::Id(string),
            ..
        }) = parser.peek().cloned()
        {
            parser.shift();
            stack.push_single(Box::new(Id(string)));
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Assignment(
    pub Box<expressions::Expression>,
    pub Box<expressions::Expression>,
);
impl expressions::Expression for Assignment {}

impl Assignment {
    pub fn new(stack: &mut stack::Stack) {
        let (varlist, _assignment, namelist) = stack_unpack!(stack, repetition, single, repetition);

        if varlist.len() != namelist.len() {
            panic!("Assignment contains different numbers of names and variables");
        }

        for (name, var) in namelist.into_iter().zip(varlist) {
            stack.push_single(Box::new(
                Assignment(name, var)
            ))
        }
    }
}
