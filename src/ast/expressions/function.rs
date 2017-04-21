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
    if tokens::Keyword::COLONS == lexer.head() {
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

// args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString
fn parse_args(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // ‘(’ [explist] ‘)’
    if lexer.skip_expected_keyword(tokens::Keyword::LBRACE, "").is_ok() {
        let explist = parse_explist(lexer);

        return lexer.skip_expected_keyword(tokens::Keyword::RBRACE, "Expected ')' at the end of arguments")
            .and(Ok(Expression::Expressions(explist)))
    }

    // tableconstructor
    if let Ok(table) = lexer.parse_or_rollback(tables::parse_table_constructor) {
        return Ok(table)
    }

    // LiteralString
    if let tokens::TokenType::String(string) = lexer.head().token {
        return Ok(Expression::String(string))
    }

    Err(error::Error::new(lexer.head(), "Expected function parameters"))
}

// Special prefixext without cyclic funcall parser :|
fn parse_special_prefixexp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.parse_or_rollback(|lexer| {
        lexer.skip_expected_keyword(tokens::Keyword::LBRACE, "")
            .and_then(|_| parse_exp(lexer))
            .and_then(|exp| lexer.skip_expected_keyword(tokens::Keyword::RBRACE, "Unclosed brace '('").map(|_| exp))

    })
        .or_else(|_| lexer.parse_or_rollback(variables::parse_var))
        .or(Err(error::Error::new(lexer.head(), "Failed to parse prefix expression")))
}

// functioncall ::=  prefixexp args | prefixexp ‘:’ Name args
pub fn parse_funcall(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    // prefixexp ‘:’ Name args
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::COLONS) {
        if let Ok(object) = sublexer.parse_all_or_rollback(parse_prefixexp) {

            lexer.skip(sublexer.pos() + 1);

            // It's sugared function call wit 'self'. Need a bit complex logic
            return if let Some(id) = lexer.head().id() {
                lexer.skip(1); // Id

                lexer.parse_or_rollback(parse_args)
                    .map(|args| {
                        let function = Expression::Indexing {
                            object: Box::new(object.clone()),
                            index: Box::new(Expression::String(id))
                        };

                        let self_args = match args {
                            Expression::Expressions(mut input_args) => {
                                let mut tmp_arg = vec![Box::new(object)];
                                // Append doesn't return vector :|
                                tmp_arg.append(&mut input_args);
                                Expression::Expressions(tmp_arg)
                            }
                            exp => Expression::Expressions(vec![Box::new(object), Box::new(exp)]),
                        };

                        Expression::Funcall {
                            function: Box::new(function),
                            args: Box::new(self_args)
                        }
                    })
            } else {
                Err(error::Error::new(lexer.head(), "Expected 'Id' after ':'"))
            }
        }
    }

    // prefixexp args
    if let Ok(func) = lexer.parse_or_rollback(parse_special_prefixexp) {
        lexer.parse_or_rollback(parse_args)
            .map(|args| {
                Expression::Funcall {
                    function: Box::new(func),
                    args: Box::new(args)
                }
            })
    } else {
        Err(error::Error::new(lexer.head(), "Exprected function call"))
    }
}
