use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error::Error;

pub type Id = String;

#[derive(Debug)]
pub struct Assignment {
    varname: Vec<Id>, // We may have complex name, e.q. a.b.c.d
    expression: Box<Expression>,
}

impl Expression for Assignment {}

impl Assignment {
    pub fn new(varname: Vec<String>, expression: Box<Expression>) -> Self {
        Assignment {
            varname: varname,
            expression: expression,
        }
    }

    pub fn parse_varname(lexer: &mut lexer::Lexer) -> Result<Vec<String>, Error> {
        let mut result = vec![];

        loop {
            if let tokens::TokenType::Id(name) = lexer.get(0).token.clone() {
                result.push(name);
                lexer.skip(1);
            } else {
                return Result::Err(Error::new(&lexer.get(0)));
            }

            if lexer.get(0).token != tokens::TokenType::Keyword(tokens::Keyword::DOT) {
                break;
            } else {
                lexer.skip(1);
            }
        }

        Result::Ok(result)
    }
}
