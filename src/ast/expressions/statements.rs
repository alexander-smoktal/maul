use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[derive(Debug)]
pub enum Statement {
    Break,
    Ellipsis,
    Return(Box<expression::Expression>),
}
impl expression::Expression for Statement {}

// retstat ::= return [explist] [‘;’]
pub fn parse_return_statement(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Return {:?}", lexer);

    lexer.skip_expected_keyword(tokens::Keyword::RETURN, "")?;

    let exps = parse_explist(lexer);

    let _ = lexer.skip_expected_keyword(
        tokens::Keyword::COLONS,
        "Expected colons to close 'return' statement",
    );

    Ok(utils::exp_box(
        Statement::Return(Box::new(common::Expressions(exps))),
    ))
}

fn parse_keyword(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Keyword {:?}", lexer);

    let exp: ParseResult = match lexer.head().keyword().unwrap() {
        tokens::Keyword::SEMICOLONS => Ok(utils::exp_box(common::Noop)),
        tokens::Keyword::BREAK => Ok(utils::exp_box(Statement::Break)),
        _ => Err(error::Error::new(lexer.head(), "Unexpected keyword: {:?}")),
    };

    exp.map(|exp| {
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
pub fn parse_statement(lexer: &mut lexer::Lexer) -> ParseResult {
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
}
