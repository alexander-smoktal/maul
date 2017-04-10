use ast::lexer::tokens;

#[derive(Debug)]
pub struct Error {
    pub error_token: tokens::Token,
}

impl Error {
    pub fn new(token: &tokens::Token) -> Self {
        Error { error_token: token.clone() }
    }

    pub fn complain(&self, message: String) {
        panic!("{}: {}. {} {:?}",
               self.error_token.row,
               self.error_token.column,
               message,
               self.error_token);
    }
}
