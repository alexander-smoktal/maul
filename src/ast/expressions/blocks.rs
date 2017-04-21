use super::*;
use ast::lexer;
use error;

// block ::= {stat} [retstat]
pub fn parse_block(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    let mut result = vec![];

    while let Ok(stat) = lexer.parse_or_rollback(statements::parse_statement) {
        if stat != Expression::Noop {
            result.push(Box::new(stat));
        }
    }

    if let Ok(retstat) = lexer.parse_or_rollback(statements::parse_return_statement) {
        result.push(Box::new(retstat))
    }

    Ok(Expression::Expressions(result))
}

// do block end
pub fn parse_do_block(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.skip_expected_keyword(tokens::Keyword::DO, "Expected 'do' keyword")
         .and_then(|_| parse_block(lexer))
         .and_then(|block| lexer.skip_expected_keyword(tokens::Keyword::END, "Expected 'end' to close a block")
                                .map(|_| Expression::DoBlock(Box::new(block))))
}
