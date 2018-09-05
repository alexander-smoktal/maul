use ast::lexer::tokens;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub error_token: tokens::Token,
    pub message: String,
}

impl Error {
    pub fn new(token: tokens::Token, message: &str) -> Self {
        Error {
            error_token: token,
            message: message.to_owned(),
        }
    }

    pub fn add(self, message: &str) -> Error {
        Error {
            error_token: self.error_token,
            message: self.message + message,
        }
    }

    pub fn complain(&self) {
        panic!(
            "{}: {}. {} {:?}",
            self.error_token.row, self.error_token.column, self.message, self.error_token
        );
    }
}
