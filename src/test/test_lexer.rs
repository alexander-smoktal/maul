#![cfg(test)]

use ast::lexer::tokens::*;
use ast::lexer::*;

#[test]
fn test_empty_lexer() {
    let iter = TokenIterator::new("".to_owned());

    for _ in iter {
        unreachable!()
    }
}


#[test]
fn test_keywords() {
    let mut iter = TokenIterator::new("and Hello or World while".to_owned());

    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Keyword(Keyword::AND), 1, 3))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Id(String::from("Hello")), 1, 9))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Keyword(Keyword::OR), 1, 12))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Id(String::from("World")), 1, 18))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Keyword(Keyword::WHILE), 1, 24))
    );
    assert_eq!(iter.next(), None);
}


#[test]
fn test_strings() {
    let mut iter = TokenIterator::new(r#""Hello" "world""!""#.to_owned());

    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::String(String::from("Hello")), 1, 7))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::String(String::from("world")), 1, 15))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::String(String::from("!")), 1, 18))
    );
    assert_eq!(iter.next(), None);
}

#[test]
#[should_panic]
fn test_invalid_strings() {
    let mut iter = TokenIterator::new(r#""Hello"#.to_owned());

    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::String(String::from("Hello")), 1, 8))
    );
}

#[test]
fn test_numbers() {
    let mut iter = TokenIterator::new("3 43 42.42 777".to_owned());

    assert_eq!(iter.next(), Some(Token::new(TokenType::Number(3f64), 1, 1)));
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Number(43f64), 1, 4))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Number(42.42f64), 1, 10))
    );
    assert_eq!(
        iter.next(),
        Some(Token::new(TokenType::Number(777f64), 1, 14))
    );
    assert_eq!(iter.next(), None);
}
