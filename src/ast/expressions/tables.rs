use error;
use ast::lexer;
use super::*;

// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
fn parse_field(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // ‘[’ exp ‘]’ ‘=’ exp
    if let Ok(expression) = lexer.try_to_parse(
        |lexer: &mut lexer::Lexer| {
            lexer.skip_expected_keyword(tokens::Keyword::LSBRACKET, "")
                .and_then(|_| lexer.try_to_parse(parse_exp))
                .and_then(|index| lexer.skip_expected_keyword(tokens::Keyword::RSBRACKET, "Expected ']' at the end of indexing").map(|_| index))
                .and_then(|index| lexer.skip_expected_keyword(tokens::Keyword::ASSIGN, "Expected assignment after table indexing expression").map(|_| index))
                .and_then(|index| lexer.try_to_parse(parse_exp).map(|value| Expression::Assignment(Box::new(index), Box::new(value))))
        }) {
        return Ok(expression)
    }

    if let Some(id) = lexer.head().id() {
        if let Ok(expression) = lexer.try_to_parse(
            |lexer| {
                lexer.skip(1);
                lexer.skip_expected_keyword(tokens::Keyword::ASSIGN, "Expected assignment after table key expression")
                    .and_then(|_| lexer.try_to_parse(parse_exp).map(|value| Expression::Assignment(Box::new(Expression::Id(vec![id.clone()])), Box::new(value))))
            }) {
            return Ok(expression)
        }
    }

    if let Ok(expression) = lexer.try_to_parse(parse_exp) {
        return Ok(expression)
    }

    Err(error::Error::new(lexer.head(), "Expected table field"))
}

// tableconstructor ::= ‘{’ [fieldlist] ‘}’
// fieldlist ::= field {fieldsep field} [fieldsep]
// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
// fieldsep ::= ‘,’ | ‘;’
pub fn parse_table_constructor(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.skip_expected_keyword(tokens::Keyword::LCBRACKET, "Expected '{' at the beginning of table constructor")?;

    let mut fields: Vec<Box<Expression>> = vec![];

    if let Ok(field) = parse_field(lexer) {
        fields.push(Box::new(field));

        loop {
            if lexer.head().keyword() == Some(tokens::Keyword::COMMA)
                || lexer.head().keyword() == Some(tokens::Keyword::COLONS) {
                lexer.skip(1);
            }

            if let Ok(field) = parse_field(lexer) {
                fields.push(Box::new(field));
            } else {
                match lexer.skip_expected_keyword(tokens::Keyword::RCBRACKET, "Expected table constructor closing brace '}'") {
                    Ok(_) => return Ok(Expression::TableConstructor(fields)),
                    err => return err.and(Ok(Expression::Stub))
                }
            }
        }
    }

    Err(error::Error::new(lexer.head(), "Expected fields inside table constructor expression"))
}
