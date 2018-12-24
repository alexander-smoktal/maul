use crate::ast::expressions;
use crate::ast::lexer::tokens;
use crate::ast::parser;
use crate::ast::stack;

use crate::interpreter;

#[derive(Debug)]
pub enum Statement {
    Break,
    Ellipsis,
    Return(Option<Box<expressions::Expression>>),
}
impl interpreter::Eval for Statement {}
impl expressions::Expression for Statement {}

impl Statement {
    make_keyword_rule![ellipsis, (tokens::Keyword::DOT3, Statement::Ellipsis)];
    make_keyword_rule![breakstat, (tokens::Keyword::BREAK, Statement::Break)];
}
