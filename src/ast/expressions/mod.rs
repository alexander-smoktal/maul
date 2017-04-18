pub mod function;
pub mod variables;
pub mod tables;
pub mod statements;
pub mod blocks;

use std::vec::Vec;

use error;
use ast::lexer;
use ast::lexer::tokens;

#[derive(PartialEq, Debug)]
pub enum Expression {
    Noop,
    Id(variables::Id),
    Assignment(Box<Expression>, Box<Expression>),
    Function {
        params: variables::Id,
        body: Box<Expression>,
    },
    Indexing {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    TableConstructor(Vec<Box<Expression>>),
    Expressions(Vec<Box<Expression>>),
    Binop(tokens::Keyword, Box<Expression>, Box<Expression>),
    Unop(tokens::Keyword, Box<Expression>),
    St(statements::Statement),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

// prefixexp ::= var | functioncall | ‘(’ exp ‘)’
pub fn parse_prefixexp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.try_to_parse(|lexer| {
        lexer.skip_expected_keyword(tokens::Keyword::LBRACE, "")
            .and_then(|_| parse_exp(lexer))
            .and_then(|exp| lexer.skip_expected_keyword(tokens::Keyword::RBRACE, "Unclosed brace '('").map(|_| exp))

    })
        .or_else(|_| lexer.try_to_parse(variables::parse_var))
        .or_else(|_| lexer.try_to_parse(function::parse_funcall))
        .or(Err(error::Error::new(lexer.head(), "Failed to parse prefix expression")))
}
// exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
//          prefixexp | tableconstructor | exp binop exp | unop exp
pub fn parse_exp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // exp binop exp
    if let Some(mut sublexer) = lexer.take_while(|t| t.keyword().map_or(false, |k| k.is_binop())) {
        if let Ok(left) = parse_exp(&mut sublexer) {
            lexer.skip(sublexer.pos());

            if let tokens::TokenType::Keyword(binop) = lexer.head().token {
                lexer.skip(1);

                if let Ok(right) = lexer.try_to_parse(parse_exp) {
                    return Ok(Expression::Binop(binop, Box::new(left), Box::new(right)))
                }
            }
        }
    }

    // unop exp
    if let tokens::TokenType::Keyword(keyword) = lexer.head().into() {
        if keyword.is_unop() {
            lexer.skip(1);

            if let Ok(exp) = lexer.try_to_parse(parse_exp) {
                return Ok(Expression::Unop(keyword, Box::new(exp)))
            }
        }
    }

    // funcdef
    if let Ok(funcdef) = lexer.try_to_parse(function::parse_funcdef) {
        return Ok(funcdef)
    }

    // prefixexp
    if let Ok(prefixexp) = lexer.try_to_parse(parse_prefixexp) {
        return Ok(prefixexp)
    }

    // tableconstructor
    if let Ok(table) = lexer.try_to_parse(tables::parse_table_constructor) {
        return Ok(table)
    }

    match lexer.head().token.clone() {
        tokens::TokenType::Keyword(tokens::Keyword::NIL) => Ok(Expression::Nil),
        tokens::TokenType::Keyword(tokens::Keyword::FALSE) => Ok(Expression::Boolean(false)),
        tokens::TokenType::Keyword(tokens::Keyword::TRUE) => Ok(Expression::Boolean(true)),
        tokens::TokenType::Keyword(tokens::Keyword::DOT3) => Ok(Expression::St(statements::Statement::Ellipsis)),
        tokens::TokenType::Number(number) => Ok(Expression::Number(number)),
        tokens::TokenType::String(string) => Ok(Expression::String(string)),
        _ => Err(error::Error::new(lexer.head(), "Unexpected token"))
    }.and_then(|x| { lexer.skip(1); Ok(x) })
}

// explist ::= exp {‘,’ exp}
fn parse_explist(lexer: &mut lexer::Lexer) -> Vec<Box<Expression>> {
    let mut result = vec![];

    while let Ok(var) = lexer.try_to_parse(parse_exp) {
        result.push(Box::new(var));

        if lexer.skip_expected_keyword(tokens::Keyword::COMMA, "").is_err() {
            break
        }
    }

    result
}

impl Expression {
    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Option<Expression> {
        blocks::parse_block(lexer).ok()
    }
}
