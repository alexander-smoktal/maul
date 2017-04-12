use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

fn parse_func_args(lexer: &mut lexer::Lexer) -> Result<assignment::Id, error::Error> {
    let mut result = vec![];

    lexer.skip_expected_keyword(tokens::Keyword::LBRACE,
                                "Expected function parameters start")?;

    while let Some(tokens::Token{ token: tokens::TokenType::Id(name), .. })
              = lexer.head().cloned() {
        result.push(name);
        lexer.skip(1);

        if lexer.head_token_is_keyword(tokens::Keyword::COMMA) {
            lexer.skip(1);
        }
    }

    lexer.skip_expected_keyword(tokens::Keyword::RBRACE,
                                "Expected ')' at the end of parameters")?;

    Ok(result)
}

fn parse_method_name(lexer: &mut lexer::Lexer) -> Result<assignment::Id, error::Error> {
    if lexer.head_token_is_keyword(tokens::Keyword::SEMICOLONS) {
        lexer.skip(1);

        if let Some(tokens::Token { token: tokens::TokenType::Id(name), .. }) = lexer.head().cloned() {
            lexer.skip(1);
            Ok(vec![name])
        } else {
            Err(error::Error::new(lexer.head_or_eof(), "Failed to parse method name"))
        }
    } else {
        Ok(vec![])
    }
}

// functiondef ::= function funcbody
// funcbody ::= ‘(’ [parlist] ‘)’ block end
// parlist ::= namelist [‘,’ ‘...’] | ‘...’
pub fn parse_funcdef(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // First parse function name as variable
    let mut function_name = assignment::parse_varname(lexer)
        .map_err(|e| e.add("Failed to parse function name"))?;

    // Then parse method name if method
    let mut params = vec![];

    let mut method_name = parse_method_name(lexer)?;

    if !method_name.is_empty() {
        function_name.append(&mut method_name);
        params.push("self".to_owned())
    }

    // Parse function arguments
    params.append(&mut parse_func_args(lexer)?);

    let func = Expression::Function {
        params: params,
        body: vec![],
    };

    // Return assignment, because of function is a sugar for var
    Ok(assignment::new(function_name, func))
}

pub fn parse_funcall(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    Err(error::Error::new(lexer.head_or_eof(), "Stub"))
}
