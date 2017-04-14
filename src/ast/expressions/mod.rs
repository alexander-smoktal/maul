pub mod function;
pub mod variables;
pub mod tables;
pub mod statements;

use std::vec::Vec;

use error;
use ast::lexer;
use ast::lexer::tokens;

use self::statements::Statement;

#[derive(PartialEq, Debug)]
pub enum Expression {
    Stub,
    Id(variables::Id),
    Assignment(Box<Expression>, Box<Expression>),
    Function {
        params: variables::Id,
        body: Expressions,
    },
    Indexing {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    TableConstructor(Vec<Box<Expression>>),
    Binop(tokens::Keyword, Box<Expression>, Box<Expression>),
    Unop(tokens::Keyword, Box<Expression>),
    St(statements::Statement),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

pub type Expressions = Vec<Expression>;

// chunk ::= block

// block ::= {stat} [retstat]

// stat ::=  ‘;’ |
//     varlist ‘=’ explist |
//     functioncall |
//     label |
//     break |
//     goto Name |
//     do block end |
//     while exp do block end |
//     repeat block until exp |
//     if exp then block {elseif exp then block} [else block] end |
//     for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
//     for namelist in explist do block end |
//     function funcname funcbody |
//     local function Name funcbody |
//     local namelist [‘=’ explist]


// prefixexp ::= var | functioncall | ‘(’ exp ‘)’
pub fn parse_prefixexp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.try_to_parse(variables::parse_var)
        .or_else(|_| lexer.try_to_parse(function::parse_funcall))
        .or(Err(error::Error::new(lexer.head(), "Failed to parse prefix expression")))
}
// exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
//          prefixexp | tableconstructor | exp binop exp | unop exp
pub fn parse_exp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    println!("NEW PARSE {:?}", lexer);

    println!("0");
    // exp binop exp
    if let Some(mut sublexer) = lexer.take_while(|t| t.keyword().map_or(false, |k| k.is_binop())) {
        if let Ok(left) = parse_exp(&mut sublexer) {
            lexer.skip(sublexer.pos());

            if let tokens::TokenType::Keyword(binop) = lexer.head().token {
                lexer.skip(1);

                println!("PARSING EXP AFTER BINOP {:?} {:?} {:?}", left, binop, lexer.head());

                if let Ok(right) = lexer.try_to_parse(parse_exp) {
                    println!("FOUND RIGHT {:?}", right);
                    return Ok(Expression::Binop(binop, Box::new(left), Box::new(right)))
                }
            }
        }
    }

    println!("1");
    // unop exp
    if let tokens::TokenType::Keyword(keyword) = lexer.head().into() {
        if keyword.is_unop() {
            lexer.skip(1);

            if let Ok(exp) = lexer.try_to_parse(parse_exp) {
                return Ok(Expression::Unop(keyword, Box::new(exp)))
            }
        }
    }

    println!("2");
    // funcdef
    if let Ok(funcdef) = lexer.try_to_parse(function::parse_funcdef) {
        return Ok(funcdef)
    }

    println!("3");
    // prefixexp
    if let Ok(prefixexp) = lexer.try_to_parse(parse_prefixexp) {
        return Ok(prefixexp)
    }

    println!("4");
    // tableconstructor
    if let Ok(table) = lexer.try_to_parse(tables::parse_table_constructor) {
        return Ok(table)
    }

    println!("5");
    match lexer.head().token.clone() {
        tokens::TokenType::Keyword(tokens::Keyword::NIL) => Ok(Expression::Nil),
        tokens::TokenType::Keyword(tokens::Keyword::FALSE) => Ok(Expression::Boolean(false)),
        tokens::TokenType::Keyword(tokens::Keyword::TRUE) => Ok(Expression::Boolean(true)),
        tokens::TokenType::Keyword(tokens::Keyword::DOT3) => Ok(Expression::St(statements::Statement::Ellipsis)),
        tokens::TokenType::Number(number) => Ok(Expression::Number(number)),
        tokens::TokenType::String(string) => Ok(Expression::String(string)),
        _ => Err(error::Error::new(lexer.head(), "Unexpected token"))
    }.and_then(|x| { lexer.skip(1); Ok(x) })
}

impl Expression {
    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Option<Expression> {
        match lexer.head().token {
            tokens::TokenType::Keyword(ref keyword) => {

                match keyword {
                    &tokens::Keyword::COLONS => Some(Expression::St(Statement::Break)),
                    &tokens::Keyword::FUNCTION => {
                        function::parse_funcdef(lexer).ok()
                    }
                    _ => panic!("Unexpected keyword: {:?}", keyword),
                }
            },
            tokens::TokenType::Id(_) => {
                variables::parse_var(lexer).ok()
            },
            tokens::TokenType::String(ref string) => {
                panic!("Unexpected Id: {}", string);
            },
            tokens::TokenType::Number(ref string) => {
                panic!("Unexpected Id: {}", string);
            },
            _ => panic!("Unexpected End of File")
        }
    }
}
