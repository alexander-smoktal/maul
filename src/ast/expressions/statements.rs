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
    lexer.skip_expected_keyword(tokens::Keyword::RETURN, "")?;

    let exps = parse_explist(lexer);

    let _ = lexer.skip_expected_keyword(tokens::Keyword::COLONS, "");

    Ok(Expression::St(Statement::Return(Box::new(Expression::Expressions(exps)))))
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
    match lexer.head().token {
        tokens::TokenType::Keyword(ref keyword) => {
            match keyword {
                &tokens::Keyword::COLONS => {
                    lexer.skip(1);
                    Ok(Expression::Noop)
                }
                &tokens::Keyword::BREAK => {
                    lexer.skip(1);
                    Ok(Expression::St(Statement::Break))
                }
                &tokens::Keyword::FUNCTION => function::parse_funcdef(lexer),
                _ => Err(error::Error::new(lexer.head(), "Unexpected keyword: {:?}")),
            }
        }
        tokens::TokenType::Id(_) => {
            variables::parse_assignment(lexer)
        }
        _ => Err(error::Error::new(lexer.head(), "Unexpected token"))
    }
}
