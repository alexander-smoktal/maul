use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

fn parse_func_args(lexer: &mut lexer::Lexer) -> assignment::Id {
    let mut result = vec![];

    lexer.skip_expected_keyword(tokens::Keyword::LBRACE,
                                "Expected function parameters start");

    while let tokens::TokenType::Id(name) = lexer.get(0).token.clone() {
        result.push(name);
        lexer.skip(1);

        if lexer.get(0).token == tokens::TokenType::Keyword(tokens::Keyword::COMMA) {
            lexer.skip(1);
        }
    }

    lexer.skip_expected_keyword(tokens::Keyword::RBRACE,
                                "Expected ')' at the end of parameters");

    result
}

fn parse_method_name(lexer: &mut lexer::Lexer) -> Option<assignment::Id> {
    if lexer.get(0).token == tokens::TokenType::Keyword(tokens::Keyword::SEMICOLONS) {
        lexer.skip(1);

        if let tokens::TokenType::Id(name) = lexer.get(0).token.clone() {
            lexer.skip(1);
            Some(vec![name])
        } else {
            error::Error::new(&lexer.get(0)).complain("Expected method name, got:".to_owned());
            unreachable!()
        }
    } else {
        None
    }
}

pub fn from_lexer(lexer: &mut lexer::Lexer) -> Expression {
    // First parse function name as variable
    let mut function_name = match assignment::parse_varname(lexer) {
        Ok(varname) => varname,
        Err(e) => {
            e.complain("Failed to parse function name, expected id, got:".to_owned());
            unreachable!()
        }
    };

    // Then parse method name if method
    let mut params = vec![];
    if let Some(mut method_name) = parse_method_name(lexer) {
        function_name.append(&mut method_name);
        params.push("self".to_owned())
    }

    // Parse function arguments
    params.append(&mut parse_func_args(lexer));

    let func = Expression::Function {
        params: params,
        body: vec![],
    };

    // Return assignment, because of function is a sugar for var
    assignment::new(function_name, func)
}

pub fn parse_funcall(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    Result::Ok(Expression::Stub)
}
