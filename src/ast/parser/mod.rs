mod tests;
pub mod expressions;

use super::lexer::Lexer;
use super::AST;

// exp ::= prefixexp
// exp ::= nil | false | true
// exp ::= Numeral
// exp ::= LiteralString
// exp ::= functiondef
// exp ::= tableconstructor
// exp ::= ‘...’
// exp ::= exp binop exp
// exp ::= unop exp
// prefixexp ::= var | functioncall | ‘(’ exp ‘)’

pub struct Parser {
    lexer: Lexer,
}

/// I'll try to make this recursive descendant parser, buy hey... nobody is perfect
impl Parser {
    pub fn new(input: String) -> Parser {
        Parser { lexer: Lexer::new(input) }
    }

    #[allow(dead_code, non_snake_case)]
    pub fn create_AST(&self) -> AST {
        for token in self.lexer.into_iter() {
            println!("Token {:?}", token)
        }

        AST { expressions: vec![] }
    }
}