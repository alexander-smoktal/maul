pub mod tokens;
mod tests;

use self::tokens::{get_token_table, get_operator_table, Token, TokenType, Keyword};

use std::vec;
use std::collections::HashMap;
use std::iter::{Iterator, IntoIterator, FromIterator, Peekable};
use std::string::String;
use std::rc::Rc;
use utils::AsExclusiveTakeWhile;

// ------------ Lexer ----------------
#[derive(Clone)]
pub struct Lexer {
    tokens: Rc<Vec<Token>>,
    position: usize,
}

impl Lexer {
    /// Create new lexer, which can be used as token iteartor
    pub fn new(input: String) -> Lexer {
        Lexer {
            tokens: Rc::new(Vec::<Token>::from_iter(TokenIterator::new(input))),
            position: 0,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.tokens.len() {
            None
        } else {
            let result = Some(self.tokens[self.position].clone());
            self.position += 1;
            result
        }
    }
}

// ---------- Token Iterator --------------
pub struct TokenIterator {
    char_iterator: Peekable<vec::IntoIter<char>>,
    token_table: HashMap<String, Keyword>,
    operator_table: HashMap<String, Keyword>,

    // Line and column
    row: usize,
    column: usize,
}

impl TokenIterator {
    pub fn new(source_code: String) -> Self {
        let chars = Vec::<char>::from_iter(source_code.chars());
        let peekable = chars.into_iter().peekable();
        TokenIterator {
            char_iterator: peekable,
            token_table: get_token_table(),
            operator_table: get_operator_table(),
            row: 1,
            column: 0,
        }
    }

    fn advance_pos(&mut self, n: usize) {
        self.column += n;
    }

    fn newline(&mut self) {
        self.row += 1;
        self.column = 0;
    }

    fn parse_next_token(&mut self) -> Option<Token> {
        if let Some(&chr) = self.char_iterator.peek() {
            if chr.is_alphabetic() || chr == '_' {
                Some(Token::new(self.parse_identifier(), self.row, self.column))
            } else if chr.is_numeric() {
                Some(Token::new(self.parse_number(), self.row, self.column))
            } else if chr == '"' {
                Some(Token::new(self.parse_string(), self.row, self.column))
            } else if chr == '\n' {
                self.char_iterator.next();
                self.newline();
                self.parse_next_token()
            } else if chr == ' ' || chr == '\t' {
                self.char_iterator.next();
                self.advance_pos(1);
                self.parse_next_token()
            } else {
                Some(Token::new(self.parse_operator(), self.row, self.column))
            }
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> TokenType {
        let id_chars = |chr: &char| chr.is_alphanumeric();

        let id: String = self.char_iterator.take_while_exclusive(id_chars).collect();
        self.advance_pos(id.len());

        // If keyword map contains the keyword, return Token::Keyword
        // Else return a Token::Identifier
        match self.token_table.get(&id) {
            Some(keyword) => return TokenType::Keyword(keyword.clone()),
            _ => return TokenType::Id(id),
        }
    }

    fn parse_string(&mut self) -> TokenType {
        // Looking for the closing doublequote
        let string_chars = |chr: &char| chr.clone() != '"';

        // Skip starting doublequote
        self.char_iterator.next();

        let string: String = self.char_iterator.take_while_exclusive(string_chars).collect();
        self.advance_pos(string.len() + 2); // With doublequotes

        // Skip ending doublequote
        if let None = self.char_iterator.next() {
            panic!("Unmatched double quotes at {}:{}", self.row, self.column)
        }

        TokenType::String(string)
    }

    fn parse_number(&mut self) -> TokenType {
        let numeric_chars = |chr: &char| chr.is_numeric() || chr.clone() == '.';

        // Looking for the end of the number
        let number: String = self.char_iterator.take_while_exclusive(numeric_chars).collect();
        self.advance_pos(number.len());

        TokenType::Number(number)
    }

    fn parse_operator(&mut self) -> TokenType {
        // First we try longer operators then shorter, to avoid returning '>' instead of '>='
        // Lenghts are 3, 2, 1
        for n_operator in (1..4).rev() {
            let n_character_operator: String =
                self.char_iterator.clone().take(n_operator).collect();
            if let Some(keyword) = self.operator_table.get(&n_character_operator).cloned() {
                // Advance original iterator. Need to coolect, to power on lazy iterartor
                let _ = self.char_iterator.by_ref().take(n_operator).count();
                self.advance_pos(n_operator);

                return TokenType::Keyword(keyword.clone());
            }
        }

        panic!("Can't parse token at {}:{}", self.row, self.column)
    }
}

// ----- Iterator trait implementation for the scanner -----
impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.parse_next_token()
    }
}
