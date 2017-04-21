use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

// label ::= ‘::’ Name ‘::’
pub fn parse_label(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer
        .skip_expected_keyword(tokens::Keyword::PATH, "Expect '::' at label start")
        .and_then(|_| {
            lexer
                .head()
                .id()
                .map_or_else(|| Err(error::Error::new(lexer.head(), "Expected Id as label name")),
                             |id| Ok(Expression::Label(id)))
        })
        .and_then(|label| {
                      lexer
                          .skip(1)
                          .skip_expected_keyword(tokens::Keyword::PATH,
                                                 "Expect '::' after a label")
                          .map(|_| label)
                  })
}

// goto Name
pub fn parse_goto(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.skip_expected_keyword(tokens::Keyword::GOTO, "Expect 'goto' keyword")
         .and_then(|_| lexer.head().id()
                   .map_or_else(|| Err(error::Error::new(lexer.head(), "Expected Id as 'goto' label name"))
                                , |id| Ok(Expression::Goto(id))))
         .map(|id| {
             lexer.skip(1);
             id
         })
}