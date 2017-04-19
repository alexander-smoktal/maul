use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

pub type Id = Vec<String>;

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

// varlist ::= var {‘,’ var}
fn parse_varlist(lexer: &mut lexer::Lexer) -> Vec<Box<Expression>> {
    let mut result = vec![];

    while let Ok(var) = lexer.parse_or_rollback(parse_var) {
        result.push(Box::new(var));

        if lexer.skip_expected_keyword(tokens::Keyword::COMMA, "").is_err() {
            break
        }
    }

    result
}

// varlist ‘=’ explist
pub fn parse_assignment(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    let vars = parse_varlist(lexer);

    lexer.skip_expected_keyword(tokens::Keyword::ASSIGN, "Expected assignment")?;

    let exps = parse_explist(lexer);

    if vars.len() == exps.len() {
        Ok(Expression::Expressions(vars.into_iter()
                                   .zip(exps.into_iter())
                                   .map(|(var, exp)| Box::new(Expression::Assignment(var, exp)))
                                   .collect()))
    } else {
        Err(error::Error::new(lexer.head(), "Mismatched variables and expression count"))
    }
}

// var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name
pub fn parse_var(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // prefixexp ‘[’ exp ‘]’
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::LSBRACKET) {
        if let Ok(object) = sublexer.parse_all_or_rollback(parse_prefixexp) {
            lexer.skip(sublexer.pos() + 1);

            if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::RSBRACKET) {
                if let Ok(index) = sublexer.parse_all_or_rollback(parse_exp) {
                    lexer.skip(sublexer.pos() + 1);

                    return Ok(Expression::Indexing {
                        object: Box::new(object),
                        index: Box::new(index)
                    })
                } else {
                    return Err(error::Error::new(lexer.head(), "Expected valid expression inside indexing statement"))
                }
            } else {
                return Err(error::Error::new(lexer.head(), "Expected ']' at the end of index expression"))
            }
        }
    }

    // prefixexp ‘.’ Name
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::DOT) {
        if let Ok(object) = sublexer.parse_all_or_rollback(parse_prefixexp) {

            lexer.skip(sublexer.pos() + 1);

            if let Some(id) = lexer.head().id() {
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
        lexer.skip(1);

        return Ok(Expression::Id(vec![id]))
    }

    Err(error::Error::new(lexer.head(), "Expected variable expression"))
}
