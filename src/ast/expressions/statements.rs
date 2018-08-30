use std::ops;

use ast::parser;
use ast::lexer::tokens;
use ast::expressions::*;

#[derive(Debug)]
pub enum Statement {
    Break,
    Ellipsis,
    Return(Box<expression::Expression>),
}
impl expression::Expression for Statement {}

impl Statement {
    make_keyword_rule![ellipsis, (tokens::Keyword::DOT3, Statement::Ellipsis)];
    make_keyword_rule![breakstat, (tokens::Keyword::BREAK, Statement::Break)];
}



// stat ::=  ‘;’ |
// varlist ‘=’ explist |
// functioncall |
// label |
// break |
// goto Name |
// do block end |
// while exp do block end |
// repeat block until exp |
// if exp then block {elseif exp then block} [else block] end |
// for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
// for namelist in explist do block end |
// function funcname funcbody |
// local function Name funcbody |
// local namelist [‘=’ explist]
/*pub fn parse_statement(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Statement {:?}", lexer);

    match lexer.head().token.clone() {
        tokens::TokenType::Keyword(tokens::Keyword::FUNCTION) => function::parse_funcdef(lexer),
        tokens::TokenType::Keyword(tokens::Keyword::PATH) => labels::parse_label(lexer),
        tokens::TokenType::Keyword(tokens::Keyword::GOTO) => labels::parse_goto(lexer),
        tokens::TokenType::Keyword(tokens::Keyword::DO) => blocks::parse_do_block(lexer),
        tokens::TokenType::Keyword(tokens::Keyword::WHILE) => blocks::parse_while_block(lexer),
        tokens::TokenType::Keyword(tokens::Keyword::REPEAT) => blocks::parse_repeat_block(lexer),
        tokens::TokenType::Keyword(tokens::Keyword::IF) => blocks::parse_if_block(lexer),
        tokens::TokenType::Keyword(_) => parse_keyword(lexer),
        tokens::TokenType::Id(_) => {
            lexer
                .parse_or_rollback(variables::parse_assignment)
                .or_else(|_| function::parse_funcall(lexer))
        }
        _ => Err(error::Error::new(lexer.head(), "Unexpected token")),
    }
}*/
