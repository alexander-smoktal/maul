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
        if let Some(name) = lexer.head().id() {
            result.push(name);
            lexer.skip(1);
        } else {
            return Err(error::Error::new(lexer.head(), "Expected variable id. "));
        }

        if tokens::Keyword::DOT != lexer.head() {
            break;
        } else {
            lexer.skip(1);
        }
    }

    Ok(result)
}

// varlist ‘=’ explist
// pub fn parse_assignment(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
//    Result::Ok(Expression::Stub)
// }

// var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name
pub fn parse_var(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // prefixexp ‘[’ exp ‘]’
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::LSBRACKET) {
        if let Ok(object) = parse_prefixexp(&mut sublexer) {
            lexer.skip(sublexer.pos() + 1);

            if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::RSBRACKET) {
                let index = parse_prefixexp(&mut sublexer)?;
                lexer.skip(sublexer.pos() + 1);

                return Ok(Expression::Indexing {
                    object: Box::new(object),
                    index: Box::new(index)
                })
            } else {
                return Err(error::Error::new(lexer.head(), "Expected ']' at the end of index expression"))
            }
        }
    }

    // prefixexp ‘.’ Name
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::DOT) {
        if let Ok(object) = parse_prefixexp(&mut sublexer) {
            lexer.skip(sublexer.pos() + 1);

            if let Some(id) = lexer.head().id() {
                print!("INDEX ID {:?}", id);
                return Ok(Expression::Indexing {
                    object: Box::new(object),
                    index: Box::new(Expression::String(id))
                })
            } else {
                return Err(error::Error::new(lexer.head(), "Expected 'Id' after addressing operator '.'"))
            }
        }
    }

    // Name
    if let Some(id) = lexer.head().id() {
        print!("JUST ID {:?}", id);
        lexer.skip(1);

        return Ok(Expression::Id(id))
    }

    Err(error::Error::new(lexer.head(), "Expected variable expression"))
}
