use error;
use ast::lexer;
use super::*;

#[derive(Debug)]
pub struct Indexing {
    pub object: Box<expression::Expression>,
    pub index: Box<expression::Expression>,
}
impl expression::Expression for Indexing {}

#[derive(Debug)]
pub struct TableConstructor(pub Vec<Box<expression::Expression>>);
impl expression::Expression for TableConstructor {}

// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
fn parse_field(lexer: &mut lexer::Lexer) -> ParseResult {
    // ‘[’ exp ‘]’ ‘=’ exp
    if let Ok(expression) = lexer.parse_or_rollback(
        |lexer: &mut lexer::Lexer| {
            lexer.skip_expected_keyword(tokens::Keyword::LSBRACKET, "")
                .and_then(|_| lexer.parse_or_rollback(parse_exp))
                .and_then(|index| lexer.skip_expected_keyword(tokens::Keyword::RSBRACKET, "Expected ']' at the end of indexing").map(|_| index))
                .and_then(|index| lexer.skip_expected_keyword(tokens::Keyword::ASSIGN, "Expected assignment after table indexing expression").map(|_| index))
                .and_then(|index| lexer.parse_or_rollback(parse_exp).map(|value| Box::new(variables::Assignment(index, value)) as Box<expression::Expression>))

        }) {
        return Ok(expression)
    }

    if let Some(id) = lexer.head().id() {
        if let Ok(expression) = lexer.parse_or_rollback(
            |lexer| {
                lexer.skip(1);
                lexer.skip_expected_keyword(tokens::Keyword::ASSIGN, "Expected assignment after table key expression")
                    .and_then(|_| lexer.parse_or_rollback(parse_exp).map(|value|
                        Box::new(variables::Assignment(Box::new(variables::Id(vec![id.clone()])), value)) as Box<expression::Expression>))
            }) {
            return Ok(expression)
        }
    }

    if let Ok(expression) = lexer.parse_or_rollback(parse_exp) {
        return Ok(expression)
    }

    Err(error::Error::new(lexer.head(), "Expected table field"))
}

// tableconstructor ::= ‘{’ [fieldlist] ‘}’
// fieldlist ::= field {fieldsep field} [fieldsep]
// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
// fieldsep ::= ‘,’ | ‘;’
pub fn parse_table_constructor(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer.skip_expected_keyword(tokens::Keyword::LCBRACKET, "Expected '{' at the beginning of table constructor")?;

    let mut fields: Vec<Box<expression::Expression>> = vec![];

    if let Ok(field) = parse_field(lexer) {
        fields.push(field);

        loop {
            if lexer.head().keyword() == Some(tokens::Keyword::COMMA)
                || lexer.head().keyword() == Some(tokens::Keyword::SEMICOLONS) {
                lexer.skip(1);
            }

            if let Ok(field) = parse_field(lexer) {
                fields.push(field);
            } else {
                match lexer.skip_expected_keyword(tokens::Keyword::RCBRACKET, "Expected table constructor closing brace '}'") {
                    Ok(_) => return Ok(Box::new(TableConstructor(fields))),
                    err => return err.and(Ok(Box::new(util::Noop)))
                }
            }
        }
    }

    Err(error::Error::new(lexer.head(), "Expected fields inside table constructor expression"))
}
