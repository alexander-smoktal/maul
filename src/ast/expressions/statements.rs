use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Break,
    Ellipsis,
    Return(Box<Expression>)
}

// retstat ::= return [explist] [‘;’]
pub fn parse_return_statement(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    log_debug!("-|- RETURN STATEMENT: {:?}", lexer);

    lexer.skip_expected_keyword(tokens::Keyword::RETURN, "")?;

    let exps = parse_explist(lexer);

    let _ = lexer.skip_expected_keyword(tokens::Keyword::COLONS, "");

    Ok(Expression::St(Statement::Return(Box::new(Expression::Expressions(exps)))))
}

fn parse_keyword(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    log_debug!("-|- KEYWORD: {:?}", lexer);

    match lexer.head().keyword().unwrap() {
        tokens::Keyword::COLONS => Ok(Expression::Noop),
        tokens::Keyword::BREAK => Ok(Expression::St(Statement::Break)),
        _ => Err(error::Error::new(lexer.head(), "Unexpected keyword: {:?}"))
    }.map(|exp| {
        lexer.skip(1);
        exp
    })
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
pub fn parse_statement(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    log_debug!("-|- STATEMENT: {:?}", lexer);

    match lexer.head().token.clone() {
        tokens::TokenType::Keyword(tokens::Keyword::FUNCTION) => function::parse_funcdef(lexer),
        tokens::TokenType::Keyword(_) => parse_keyword(lexer),
        tokens::TokenType::Id(_) => variables::parse_assignment(lexer),
        _ => Err(error::Error::new(lexer.head(), "Unexpected token"))
    }
}
