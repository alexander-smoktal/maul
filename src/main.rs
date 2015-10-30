pub mod utils;
mod ast;

use ast::lexer_module::lexer;
//use ast::lexer_module::tokens::*;

// To avoid warnings in tests
#[allow(dead_code, unused_variables)]
fn main()
{
    let lex = lexer::Lexer::new("and Hello or World while".to_owned());
    let iter = lex.into_iter();

    // assert_eq!(iter.next(), Some(Token::new(TokenType::Keyword(Keyword::AND), 1, 4)));
    // assert_eq!(iter.next(), Some(Token::new(TokenType::Id("Hello".as_bytes()), 1, 10)));
    // assert_eq!(iter.next(), Some(Token::new(TokenType::Keyword(Keyword::OR), 1, 13)));
    // assert_eq!(iter.next(), Some(Token::new(TokenType::Id("World".as_bytes()), 1, 19)));
    // assert_eq!(iter.next(), Some(Token::new(TokenType::Keyword(Keyword::WHILE), 1, 25)));
    // assert_eq!(iter.next(), None);
}
