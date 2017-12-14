use super::*;
use ast::lexer;

#[derive(Debug)]
pub struct DoBlock(pub Box<expression::Expression>);
impl expression::Expression for DoBlock {}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Box<expression::Expression>,
    pub block: Box<expression::Expression>,
}
impl expression::Expression for WhileBlock {}

#[derive(Debug)]
pub struct RepeatBlock {
    pub block: Box<expression::Expression>,
    pub condition: Box<expression::Expression>,
}
impl expression::Expression for RepeatBlock {}

// We could make typedef for 'while' and 'repeat', but can't implement trait for type
#[cfg(test)]
#[derive(Debug)]
pub struct Condition {
    pub condition: Box<expression::Expression>,
    pub block: Box<expression::Expression>,
}

#[cfg(not(test))]
#[derive(Debug)]
struct Condition {
    pub condition: Box<expression::Expression>,
    pub block: Box<expression::Expression>,
}

#[derive(Debug)]
pub struct IfBlock {
    pub conditions: Vec<Condition>,
    pub elseblock: Option<Box<expression::Expression>>,
}
impl expression::Expression for IfBlock {}

// block ::= {stat} [retstat]
pub fn parse_block(lexer: &mut lexer::Lexer) -> ParseResult {
    let mut result = vec![];

    result.push(statements::parse_statement(lexer)?);

    while let Ok(stat) = lexer.parse_or_rollback(statements::parse_statement) {
        result.push(stat);
    }

    if let Ok(retstat) = lexer.parse_or_rollback(statements::parse_return_statement) {
        result.push(retstat)
    }

    Ok(Box::new(util::Expressions(result)))
}

// do block end
pub fn parse_do_block(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer
        .skip_expected_keyword(tokens::Keyword::DO, "Expected 'do' keyword")
        .and_then(|_| parse_block(lexer))
        .and_then(|block| {
            lexer
                .skip_expected_keyword(tokens::Keyword::END, "Expected 'end' to close a block")
                .map(|_| Box::new(DoBlock(block)) as Box<expression::Expression>)
        })
}

// while exp do block end
pub fn parse_while_block(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer
        .skip_expected_keyword(tokens::Keyword::WHILE, "Expected 'while' keyword")
        .and_then(|_| parse_exp(lexer))
        .and_then(|condition| {
            parse_do_block(lexer).map(|block| {
                Box::new(WhileBlock { condition, block }) as Box<expression::Expression>
            })
        })
}

// repeat block until exp
pub fn parse_repeat_block(lexer: &mut lexer::Lexer) -> ParseResult {
    lexer
        .skip_expected_keyword(tokens::Keyword::REPEAT, "Expected 'repeat' keyword")
        .and_then(|_| parse_block(lexer))
        .and_then(|block| {
            lexer
                .skip_expected_keyword(
                    tokens::Keyword::UNTIL,
                    "Expected 'until' keyword to close 'repeat' block",
                )
                .and_then(|_| {
                    parse_exp(lexer).map(|condition| {
                        Box::new(RepeatBlock { block, condition }) as Box<expression::Expression>
                    })
                })
        })
}

fn parse_if_condition(lexer: &mut lexer::Lexer) -> Result<Condition, error::Error> {
    parse_exp(lexer).and_then(|condition| {
        lexer
            .skip_expected_keyword(
                tokens::Keyword::THEN,
                "Expected 'then' inside 'if/elseif' statement",
            )
            .and_then(|_| {
                parse_block(lexer).map(|block| Condition { condition, block })
            })
    })
}

// if exp then block {elseif exp then block} [else block] end |
pub fn parse_if_block(lexer: &mut lexer::Lexer) -> ParseResult {
    let mut conditions: Vec<Condition> = vec![];

    // Parse start condition
    conditions.push(lexer
        .skip_expected_keyword(tokens::Keyword::IF, "Expected 'if' keyword")
        .and_then(|_| parse_if_condition(lexer))?);

    // Parse elseifs
    loop {
        if lexer.head().token == tokens::TokenType::Keyword(tokens::Keyword::ELSEIF) {
            conditions.push(lexer
                .skip_expected_keyword(tokens::Keyword::ELSEIF, "Expected 'elseif' keyword")
                .and_then(|_| parse_if_condition(lexer))?);
        } else {
            break;
        }
    }

    // Parse else
    let mut elseblock: Option<Box<expression::Expression>> = None;

    if lexer.head().token == tokens::TokenType::Keyword(tokens::Keyword::ELSE) {
        elseblock = Some(lexer
            .skip_expected_keyword(tokens::Keyword::ELSE, "Expected 'else' keyword")
            .and_then(|_| parse_block(lexer))?);
    }

    lexer.skip_expected_keyword(
        tokens::Keyword::END,
        "Expected 'end' keyword, to close 'if' block",
    )?;

    Ok(Box::new(IfBlock {
        conditions,
        elseblock,
    }) as Box<expression::Expression>)
}
