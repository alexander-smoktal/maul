use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[derive(Debug)]
pub struct Label(pub String);
impl expression::Expression for Label {}

#[derive(Debug)]
pub struct Goto(pub String);
impl expression::Expression for Goto {}

// label ::= ‘::’ Name ‘::’
pub fn parse_label(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer
        .skip_expected_keyword(tokens::Keyword::PATH, "Expect '::' at label start")
        .and_then(|_| {
            lexer
                .head()
                .id()
                .map_or_else(|| Err(error::Error::new(lexer.head(), "Expected Id as label name")),
                             |id| Ok(Label(id)))
        })
        .and_then(|label| {
            lexer
                .skip(1)
                .skip_expected_keyword(tokens::Keyword::PATH, "Expect '::' after a label")
                .map(|_| Box::new(label) as Box<expression::Expression>)
        })
}

// goto Name
pub fn parse_goto(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer
        .skip_expected_keyword(tokens::Keyword::GOTO, "Expect 'goto' keyword")
        .and_then(|_| {
            lexer.head().id().map_or_else(
                || {
                    Err(error::Error::new(
                        lexer.head(),
                        "Expected Id as 'goto' label name",
                    ))
                },
                |id| Ok(labels::Goto(id)),
            )
        })
        .map(|id| {
            lexer.skip(1);
            Box::new(id) as Box<expression::Expression>
        })
}
