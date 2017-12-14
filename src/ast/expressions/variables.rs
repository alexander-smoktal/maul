use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[derive(Debug, Clone)]
pub struct Id(pub Vec<String>);
impl expression::Expression for Id {
    fn clone(&self) -> Box<expression::Expression> {
        Box::new(Id(self.0.clone()))
    }
}

#[derive(Debug)]
pub struct Assignment(pub Box<expression::Expression>, pub Box<expression::Expression>);
impl expression::Expression for Assignment {}

pub fn parse_varname(lexer: &mut lexer::Lexer) -> Result<Id, error::Error> {
    log_debug!("Varname {:?}", lexer);

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

    Ok(Id(result))
}

// varlist ::= var {‘,’ var}
fn parse_varlist(lexer: &mut lexer::Lexer) -> Vec<Box<expression::Expression>> {
    log_debug!("Varlist {:?}", lexer);

    let mut result = vec![];

    while let Ok(var) = lexer.parse_or_rollback(parse_var) {
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

// varlist ‘=’ explist
pub fn parse_assignment(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Assignment {:?}", lexer);

    let vars = parse_varlist(lexer);

    lexer.skip_expected_keyword(
        tokens::Keyword::ASSIGN,
        "Expected assignment",
    )?;

    let exps = parse_explist(lexer);

    if vars.len() == exps.len() {
        Ok(utils::exp_box(common::Expressions(
            vars.into_iter()
                .zip(exps.into_iter())
                .map(|(var, exp)| utils::exp_box(variables::Assignment(var, exp)))
                .collect(),
        )))
    } else {
        Err(error::Error::new(
            lexer.head(),
            "Mismatched variables and expression count",
        ))
    }
}

// var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name
pub fn parse_var(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Variable {:?}", lexer);

    // prefixexp ‘[’ exp ‘]’
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::LSBRACKET) {
        if let Ok(object) = sublexer.parse_all_or_rollback(parse_prefixexp) {
            lexer.skip(sublexer.pos() + 1);

            if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::RSBRACKET) {
                if let Ok(index) = sublexer.parse_all_or_rollback(parse_exp) {
                    lexer.skip(sublexer.pos() + 1);

                    return Ok(utils::exp_box(tables::Indexing { object, index }));
                } else {
                    return Err(error::Error::new(
                        lexer.head(),
                        "Expected valid expression inside indexing statement",
                    ));
                }
            } else {
                return Err(error::Error::new(
                    lexer.head(),
                    "Expected ']' at the end of index expression",
                ));
            }
        }
    }

    println!("STARTING TOKTYPE: {:?}", lexer);
    // prefixexp ‘.’ Name
    if let Some(mut sublexer) =
        lexer
        // Get all var prefix
        .take_while(|tok|
            match tok.token {
                tokens::TokenType::Keyword(ref kv) if kv == &tokens::Keyword::DOT => true,
                //tokens::TokenType::Id(_) => true,
                _ => false
            })
    {
        if let Ok(object) = sublexer.parse_all_or_rollback(parse_prefixexp) {
            lexer.skip(sublexer.pos() + 1);

            if let Some(id) = lexer.head().id() {
                // Skipping id
                lexer.skip(1);

                return Ok(utils::exp_box(tables::Indexing {
                    object,
                    index: Box::new(primitives::String(id)),
                }));
            } else {
                return Err(error::Error::new(
                    lexer.head(),
                    "Expected 'Id' after addressing operator '.'",
                ));
            }
        }
    }

    // Name
    if let Some(id) = lexer.head().id() {
        lexer.skip(1);

        return Ok(utils::exp_box(variables::Id(vec![id])));
    }

    Err(error::Error::new(
        lexer.head(),
        "Expected variable expression",
    ))
}
