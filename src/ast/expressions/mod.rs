pub mod function;
pub mod variables;
pub mod tables;
pub mod statements;
pub mod blocks;
pub mod labels;
pub mod primitives;
pub mod expression;
pub mod operators;

use std::vec::Vec;

use error;
use ast::lexer;
use ast::lexer::tokens;

pub type ParseResult = Result<Box<expression::Expression>, error::Error>;

pub mod util {
    use super::expression;

    #[derive(Debug)]
    pub struct Noop;
    impl expression::Expression for Noop {}

    #[derive(Debug)]
    pub struct Expressions(pub Vec<Box<expression::Expression>>);

    impl Expressions {
        pub fn prepend(&mut self, exp: Box<expression::Expression>) {
            let mut new_expressions = vec![exp];
            new_expressions.append(&mut self.0);

            self.0 = new_expressions
        }

        pub fn append(&mut self, exp: Box<expression::Expression>) {
            self.0.push(exp)
        }
    }

    impl expression::Expression for Expressions {
        fn into_expressions(self: Box<Self>) -> Box<Expressions> {
            self as Box<Expressions>
        }
    }
}

// prefixexp ::= var | functioncall | ‘(’ exp ‘)’
pub fn parse_prefixexp(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer
        .parse_or_rollback(|lexer| {
            lexer
                .skip_expected_keyword(tokens::Keyword::LBRACE, "")
                .and_then(|_| parse_exp(lexer))
                .and_then(|exp| {
                    lexer
                        .skip_expected_keyword(tokens::Keyword::RBRACE, "Unclosed brace '('")
                        .map(|_| exp)
                })

        })
        .or_else(|_| lexer.parse_or_rollback(variables::parse_var))
        .or_else(|_| lexer.parse_or_rollback(function::parse_funcall))
        .or(Err(error::Error::new(
            lexer.head(),
            "Failed to parse prefix expression",
        )))
}

// exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
//          prefixexp | tableconstructor | exp binop exp | unop exp
pub fn parse_exp(lexer: &mut lexer::Lexer) -> ParseResult {
    // exp binop exp
    if let Some(mut sublexer) = lexer.take_while(|t| t.keyword().map_or(false, |k| k.is_binop())) {
        if let Ok(left) = sublexer.parse_all_or_rollback(parse_exp) {
            lexer.skip(sublexer.pos());

            if let tokens::TokenType::Keyword(binop) = lexer.head().token {
                lexer.skip(1);

                if let Ok(right) = lexer.parse_or_rollback(parse_exp) {
                    return Ok(Box::new(operators::Binop(binop, left, right)));
                }
            }
        }
    }

    // unop exp
    if let tokens::TokenType::Keyword(keyword) = lexer.head().into() {
        if keyword.is_unop() {
            lexer.skip(1);

            if let Ok(exp) = lexer.parse_or_rollback(parse_exp) {
                return Ok(Box::new(operators::Unop(keyword, exp)));
            }
        }
    }

    // funcdef
    if let Ok(funcdef) = lexer.parse_or_rollback(function::parse_funcdef) {
        return Ok(funcdef);
    }

    // prefixexp
    if let Ok(prefixexp) = lexer.parse_or_rollback(parse_prefixexp) {
        return Ok(prefixexp);
    }

    // tableconstructor
    if let Ok(table) = lexer.parse_or_rollback(tables::parse_table_constructor) {
        return Ok(table);
    }

    let exp: ParseResult = match lexer.head().token.clone() {
        tokens::TokenType::Keyword(tokens::Keyword::NIL) => Ok(Box::new(primitives::Nil)),
        tokens::TokenType::Keyword(tokens::Keyword::FALSE) => Ok(
            Box::new(primitives::Boolean(false)),
        ),
        tokens::TokenType::Keyword(tokens::Keyword::TRUE) => Ok(
            Box::new(primitives::Boolean(true)),
        ),
        tokens::TokenType::Keyword(tokens::Keyword::DOT3) => Ok(Box::new(
            statements::Statement::Ellipsis,
        )),
        tokens::TokenType::Number(number) => Ok(Box::new(primitives::Number(number))),
        tokens::TokenType::String(string) => Ok(Box::new(primitives::String(string))),
        _ => Err(error::Error::new(lexer.head(), "Unexpected token")),
    };

    exp.and_then(|x| {
        lexer.skip(1);
        Ok(x)
    })
}

// explist ::= exp {‘,’ exp}
fn parse_explist(lexer: &mut lexer::Lexer) -> Vec<Box<expression::Expression>> {
    let mut result = vec![];

    while let Ok(var) = lexer.parse_or_rollback(parse_exp) {
        result.push(var);

        if lexer
            .skip_expected_keyword(tokens::Keyword::COMMA, "")
            .is_err()
        {
            break;
        }
    }

    result
}
