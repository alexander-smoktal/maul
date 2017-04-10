use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

pub type Id = Vec<String>;

pub fn new(varname: Id, expression: Expression) -> Expression {
    Expression::Assignment {
        varname: varname,
        expression: Box::new(expression),
    }
}

pub fn parse_varname(lexer: &mut lexer::Lexer) -> Result<Id, error::Error> {
    let mut result = vec![];

    loop {
        if let tokens::TokenType::Id(name) = lexer.get(0).token.clone() {
            result.push(name);
            lexer.skip(1);
        } else {
            return Result::Err(error::Error::new(&lexer.get(0)));
        }

        if lexer.get(0).token != tokens::TokenType::Keyword(tokens::Keyword::DOT) {
            break;
        } else {
            lexer.skip(1);
        }
    }

    Result::Ok(result)
}

// varlist ‘=’ explist
pub fn parse_assignment(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    Result::Ok(Expression::Stub)
}

// var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name
pub fn parse_var(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    match parse_prefixexp(lexer) {
        Result::Ok(expression) => {
            match lexer.get(0).token.clone() {
                tokens::TokenType::Keyword(tokens::Keyword::LSBRACKET) => {
                    lexer.skip(1);

                    if let Result::Ok(index) = parse_exp(lexer) {


                        lexer.skip_expected_keyword(tokens::Keyword::RSBRACKET, "Expected ']'");
                        Result::Ok(Expression::Indexing {
                            object: Box::new(expression),
                            index: Box::new(index)
                        })
                    } else {
                        error::Error::new(&lexer.get(0)).complain("Expected indexing expression".to_owned());

                        unreachable!()
                    }
                },
                tokens::TokenType::Keyword(tokens::Keyword::DOT) => {
                    lexer.skip(1);
                    if let tokens::TokenType::String(fieldname) = lexer.get(0).token.clone() {
                        lexer.skip(1);
                        Result::Ok(Expression::Indexing {
                            object: Box::new(expression),
                            index: Box::new(Expression::Id(fieldname))
                        })
                    } else {
                        error::Error::new(&lexer.get(0)).complain("Expected field id, got:".to_owned());
                        unreachable!()
                    }
                },
                _ => Result::Ok(expression)
            }
        },
        error => {
            error
        }
    }
}
