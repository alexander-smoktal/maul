use super::*;
use ast::lexer;
use ast::lexer::tokens;
use error;

#[derive(Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Box<expression::Expression>,
}
impl expression::Expression for Function {}

#[derive(Debug)]
pub struct Funcall {
    pub function: Box<expression::Expression>,
    pub args: Box<expression::Expression>,
}
impl expression::Expression for Funcall {}

fn parse_func_args(lexer: &mut lexer::Lexer) -> Result<variables::Id, error::Error> {
    log_debug!("Func args {:?}", lexer);

    let mut result = vec![];

    lexer.skip_expected_keyword(
        tokens::Keyword::LBRACE,
        "Expected function parameters start",
    )?;

    while let Some(name) = lexer.head().id() {
        result.push(name);
        lexer.skip(1);

        if tokens::Keyword::COMMA == lexer.head() {
            lexer.skip(1);
        }
    }

    lexer.skip_expected_keyword(
        tokens::Keyword::RBRACE,
        "Expected ')' at the end of parameters",
    )?;

    Ok(variables::Id(result))
}

fn parse_method_name(lexer: &mut lexer::Lexer) -> Result<variables::Id, error::Error> {
    log_debug!("Method name {:?}", lexer);

    if tokens::Keyword::COLONS == lexer.head() {
        lexer.skip(1);

        if let Some(name) = lexer.head().id() {
            lexer.skip(1);
            Ok(variables::Id(vec![name]))
        } else {
            Err(error::Error::new(
                lexer.head(),
                "Failed to parse method name",
            ))
        }
    } else {
        Ok(variables::Id(vec![]))
    }
}

// functiondef ::= function funcbody
// funcbody ::= ‘(’ [parlist] ‘)’ block end
// parlist ::= namelist [‘,’ ‘...’] | ‘...’
pub fn parse_funcdef(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Funcdef {:?}", lexer);

    lexer.skip_expected_keyword(
        tokens::Keyword::FUNCTION,
        "Expected 'function' keyword at the function start",
    )?;

    // First parse function name as variable
    let mut function_name = variables::parse_varname(lexer).map_err(|e| {
        e.add("Failed to parse function name")
    })?;

    // Then parse method name if method
    let mut params = vec![];

    let mut method_name = parse_method_name(lexer)?;

    if !method_name.0.is_empty() {
        function_name.0.append(&mut method_name.0);
        params.push("self".to_owned())
    }

    // Parse function arguments
    params.append(&mut parse_func_args(lexer)?.0);

    blocks::parse_block(lexer).and_then(|body| {
        println!("PARSED FUNC BODY: {:?}. LEXER: {:?}", body, lexer);

        lexer.skip_expected_keyword(
            tokens::Keyword::END,
            "Expected 'end' to close function body",
        )?;

        let func = Function { params, body };

        log_debug!("FUNC {:?}", func);

        // Return assignment, because function definition is and assignment
        Ok(utils::exp_box(variables::Assignment(
            Box::new(function_name),
            Box::new(func),
        )))
    })


}

// args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString
fn parse_args(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Args {:?}", lexer);

    // ‘(’ [explist] ‘)’
    if lexer
        .skip_expected_keyword(tokens::Keyword::LBRACE, "")
        .is_ok()
    {
        let explist = parse_explist(lexer);

        return lexer
            .skip_expected_keyword(
                tokens::Keyword::RBRACE,
                "Expected ')' at the end of arguments",
            )
            .and(Ok(utils::exp_box(common::Expressions(explist))));
    }

    // tableconstructor
    if let Ok(table) = lexer.parse_or_rollback(tables::parse_table_constructor) {
        return Ok(utils::exp_box(common::Expressions(vec![table])));
    }

    // LiteralString
    if let tokens::TokenType::String(string) = lexer.head().token {
        let string_arg = Box::new(primitives::String(string));

        return Ok(utils::exp_box(common::Expressions(vec![string_arg])));
    }

    Err(error::Error::new(
        lexer.head(),
        "Expected function parameters",
    ))
}

// Special prefixext without cyclic funcall parser :|
// prefixexp ::= var | ‘(’ exp ‘)’
// var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name
fn parse_special_prefixexp(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("S prefixexp {:?}", lexer);

    lexer
        .parse_or_rollback(|lexer| {
            lexer
                .skip_expected_keyword(tokens::Keyword::LBRACE, "")
                .and_then(|_| parse_exp(lexer))
                .and_then(|exp| {
                    lexer
                        .skip_expected_keyword(tokens::Keyword::RBRACE, "Unclosed brace '('")
                        .map(|_| exp)
                })

        })
        .or_else(|_| lexer.parse_or_rollback(variables::parse_var))
        .or(Err(error::Error::new(
            lexer.head(),
            "Failed to parse prefix expression",
        )))
}

// functioncall ::=  prefixexp args | prefixexp ‘:’ Name args
pub fn parse_funcall(lexer: &mut lexer::Lexer) -> ParseResult {
    log_debug!("Funcall {:?}", lexer);

    // prefixexp ‘:’ Name args
    if let Some(mut sublexer) = lexer.take_while_keyword(tokens::Keyword::COLONS) {
        if let Ok(object) = sublexer.parse_all_or_rollback(parse_prefixexp) {

            lexer.skip(sublexer.pos() + 1);

            // It's sugared function call wit 'self'. Need a bit complex logic
            return if let Some(id) = lexer.head().id() {
                lexer.skip(1); // Id

                lexer.parse_or_rollback(parse_args).map(|args| {
                    let function = Box::new(tables::Indexing {
                        object: object.clone(),
                        index: Box::new(primitives::String(id)),
                    });

                    let mut expressions = args.into_expressions();
                    // Add `self` argument
                    expressions.prepend(object);

                    utils::exp_box(Funcall {
                        function,
                        args: expressions,
                    })
                })
            } else {
                Err(error::Error::new(lexer.head(), "Expected 'Id' after ':'"))
            };
        }
    }

    // prefixexp args
    if let Ok(function) = lexer.parse_or_rollback(parse_special_prefixexp) {
        lexer.parse_or_rollback(parse_args).map(|args| {
            utils::exp_box(Funcall { function, args })
        })
    } else {
        Err(error::Error::new(lexer.head(), "Exprected function call"))
    }
}
