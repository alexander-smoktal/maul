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
#[derive(Debug)]
pub struct Condition {
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
    log_debug!("Block {:?}", lexer);

    /* We may have empty block or fail to parse statement. We could just check if next token
     * is block end token, in this case we'll not try to parse statement and return empty block.
     * If not the end of the block and we fail to parse statement, return verbose error
     * about statement problem */
    const BLOCK_END_STATEMENT: [tokens::Keyword; 6] = [
        tokens::Keyword::END,
        tokens::Keyword::UNTIL,
        tokens::Keyword::ELSEIF,
        tokens::Keyword::ELSE,
        tokens::Keyword::ELSEIF,
        tokens::Keyword::RETURN,
    ];

    let mut result = vec![];

    loop {
        if let tokens::TokenType::Keyword(ref ttype) = lexer.head().token {
            // This is the end of the block
            if BLOCK_END_STATEMENT.contains(ttype) {
                break;
            }
        }

        result.push(statements::parse_statement(lexer)?)
    }

    // Check if have 'return' statement
    if let Ok(retstat) = lexer.parse_or_rollback(statements::parse_return_statement) {
        result.push(retstat)
    }

    Ok(utils::exp_box(common::Expressions(result)))
}

// do block end
pub fn parse_do_block(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Do block {:?}", lexer);

    lexer
        .skip_expected_keyword(tokens::Keyword::DO, "Expected 'do' keyword")
        .and_then(|_| parse_block(lexer))
        .and_then(|block| {
            lexer
                .skip_expected_keyword(tokens::Keyword::END, "Expected 'end' to close a block")
                .map(|_| utils::exp_box(DoBlock(block)))
        })
}

// while exp do block end
pub fn parse_while_block(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("While block {:?}", lexer);

    lexer
        .skip_expected_keyword(tokens::Keyword::WHILE, "Expected 'while' keyword")
        .and_then(|_| parse_exp(lexer))
        .and_then(|condition| {
            parse_do_block(lexer).map(|block| utils::exp_box(WhileBlock { condition, block }))
        })
}

// repeat block until exp
pub fn parse_repeat_block(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Repeat block {:?}", lexer);

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
                        utils::exp_box(RepeatBlock { block, condition })
                    })
                })
        })
}

fn parse_if_condition(lexer: &mut lexer::Lexer) -> Result<Condition, error::Error> {
    log_debug!("If condition {:?}", lexer);

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
    log_debug!("If block {:?}", lexer);

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

    Ok(utils::exp_box(IfBlock {
        conditions,
        elseblock,
    }))
}
