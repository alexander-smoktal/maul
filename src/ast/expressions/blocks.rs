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
