use ast::parser;
use ast::stack;
use ast::lexer::tokens;
use ast::expressions::*;

#[derive(Debug)]
pub enum Statement {
    Break,
    Ellipsis,
    Return(Option<Box<expression::Expression>>),
}
impl expression::Expression for Statement {}

impl Statement {
    make_keyword_rule![ellipsis, (tokens::Keyword::DOT3, Statement::Ellipsis)];
    make_keyword_rule![breakstat, (tokens::Keyword::BREAK, Statement::Break)];
}
