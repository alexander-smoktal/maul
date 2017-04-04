use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error::Error;

pub type Id = Vec<String>;

pub fn new(varname: Id, expression: Expression) -> Expression {
    Expression::Assignment {
        varname: varname,
        expression: Box::new(expression),
    }
}

pub fn parse_varname(lexer: &mut lexer::Lexer) -> Result<Id, Error> {
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
