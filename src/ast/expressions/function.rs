use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

fn parse_func_args(lexer: &mut lexer::Lexer) -> Result<variables::Id, error::Error> {
    let mut result = vec![];

    lexer.skip_expected_keyword(tokens::Keyword::LBRACE,
                                "Expected function parameters start")?;

    while let Some(name) = lexer.head().id() {
        result.push(name);
        lexer.skip(1);

        if tokens::Keyword::COMMA == lexer.head() {
            lexer.skip(1);
        }
    }

    lexer.skip_expected_keyword(tokens::Keyword::RBRACE,
                                "Expected ')' at the end of parameters")?;

    Ok(result)
}

fn parse_method_name(lexer: &mut lexer::Lexer) -> Result<variables::Id, error::Error> {
    if tokens::Keyword::SEMICOLONS == lexer.head() {
        lexer.skip(1);

        if let Some(name) = lexer.head().id() {
            lexer.skip(1);
            Ok(vec![name])
        } else {
            Err(error::Error::new(lexer.head(), "Failed to parse method name"))
        }
    } else {
        Ok(vec![])
    }
}

// functiondef ::= function funcbody
// funcbody ::= ‘(’ [parlist] ‘)’ block end
// parlist ::= namelist [‘,’ ‘...’] | ‘...’
pub fn parse_funcdef(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    log_debug!("-|- FUNDEF: {:?}", lexer);

    lexer.skip_expected_keyword(tokens::Keyword::FUNCTION, "Expected 'function' keyword at the function start")?;

    // First parse function name as variable
    let mut function_name = variables::parse_varname(lexer)
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

    lexer.parse_or_rollback(blocks::parse_block).and_then(|body| {
        log_debug!("PARSED FUNC BODY: {:?}. LEXER: {:?}", body, lexer);

        lexer.skip_expected_keyword(tokens::Keyword::END, "Expected 'end' to close function body")?;

        let func = Expression::Function {
            params: params,
            body: Box::new(body),
        };

        println!("FUNC {:?}", func);

        // Return assignment, because function definition is and assignment
        Ok(Expression::Assignment(Box::new(Expression::Id(function_name)), Box::new(func)))
    })


}

pub fn parse_funcall(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    Err(error::Error::new(lexer.head(), "Stub"))
}
