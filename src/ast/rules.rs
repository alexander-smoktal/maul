use std::collections::VecDeque;

use ast::parser;
use ast::stack;
use ast::expressions::*;

use ast::lexer::tokens::Keyword;

const DEBUG: bool = true;

fn ignore(_: &mut stack::Stack) -> bool {
    true
}

fn second(stack: &mut stack::Stack) {
    let (second, _first) = stack_unpack!(stack, single, single);
    stack.push_single(second);
}

/// Function prepends element on stack before last to the last vector-element (for varlist and namelist)
fn prepend_vector_prefix(stack: &mut stack::Stack) {
    let (mut tail, head) = stack_unpack!(stack, repetition, single);
    tail.push_front(head);

    stack.push_repetition(tail);
}

// chunk ::= block
rule!(chunk, block);

//block ::= {stat} [retstat]
rule!(block, and![(repetition!(stat), retstat) => blocks::Block::new]);

/*stat ::=  ‘;’ |
        varlist ‘=’ explist |
        functioncall |
        label |
        break |
        goto Name |
        do block end |
        while exp do block end |
        repeat block until exp |
        if exp then block {elseif exp then block} [else block] end |
        for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
        for namelist in explist do block end |
        function funcname funcbody |
        local function Name funcbody |
        local namelist [‘=’ explist] !!!*/

rule!(stat, or![
    label,
    statements::Statement::breakstat,
    and![(terminal!(Keyword::GOTO), variables::Id::rule) => labels::Goto::new]
]);

// retstat ::= return [explist] [‘;’]
rule!(retstat, and![(terminal!(Keyword::RETURN),
                    optional!(and![(explist) => expression::Expressions::new], nil),
                    optional!(terminal!(Keyword::SEMICOLONS), nil)) =>
                    |stack: &mut stack::Stack| {
                        let (_semi, explist, _ret) = stack_unpack!(stack, optional, optional, single);
                        stack.push_single(Box::new(statements::Statement::Return(explist)))
                    }]);

// label ::= ‘::’ Name ‘::’
rule!(label, and![(terminal!(Keyword::PATH), variables::Id::rule, terminal!(Keyword::PATH)) => labels::Label::new]);

// funcname ::= Name {‘.’ Name} [‘:’ Name]
rule!(funcname,
    and![(
        and![(variables::Id::rule,
            repetition!(and![(terminal!(Keyword::DOT), variables::Id::rule) => second])) => prepend_vector_prefix],
        optional!(and![(terminal!(Keyword::COLONS), variables::Id::rule) => second], nil)) =>
        function::Funcname::new]);

// varlist ::= var {‘,’ var}
// We push vector on top to check assignment parity
rule!(varlist, and![(
    var,
    repetition!(and![(terminal!(Keyword::COMMA), var) => second])) =>
    prepend_vector_prefix]);

// var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
rule!(var_suffix, or![
    and![(and![(terminal!(Keyword::LSBRACKET), exp, terminal!(Keyword::RSBRACKET)) =>
        tables::Indexing::new_table], optional!(var_suffix)) => ignore],
    and![(and![(terminal!(Keyword::DOT), variables::Id::rule) =>
        tables::Indexing::new_object], optional!(var_suffix)) => ignore]
]);

// var_repetition ::= var_suffix [var_repetition] | functioncall_suffix var_suffix [var_repetition]
rule!(var_repetition, or![
    and![(var_suffix, optional!(var_repetition)) => ignore],
    and![(functioncall_suffix, var_suffix, optional!(var_repetition)) => ignore]
]);

// var ::=  Name [var_repetition] | ‘(’ exp ‘)’ var_repetition
rule!(var, or![
    and![(variables::Id::rule, optional!(var_repetition)) => ignore],
    and![(
        and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE)) =>
            |stack: &mut stack::Stack| {
                // Remove braces from stack
                let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
                stack.push_single(expression)
            }],
        var_repetition) => ignore]]);


// namelist ::= Name {‘,’ Name}
// We push vector on top to check assignment parity
rule!(namelist, and![(
    variables::Id::rule,
    repetition!(and![(terminal!(Keyword::COMMA), variables::Id::rule) => second])) =>
    prepend_vector_prefix]);

// explist ::= exp {‘,’ exp}
rule!(explist, and![(
    exp,
    repetition!(and![(
        terminal!(Keyword::COMMA),
        exp) =>
        second])) =>
    prepend_vector_prefix]);

// exp_suffix ::= binop [exp_suffix]
rule!(exp_suffix, and![(binop, optional!(exp)) => ignore]);

// exp_prefix ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
//        prefixexp | tableconstructor | unop exp !!!!
rule!(exp_prefix, or![
    primitives::Nil::rule,
    primitives::Boolean::rule,
    primitives::Number::rule,
    primitives::String::rule,
    statements::Statement::ellipsis,
    unop
]);

// exp ::= exp_prefix [exp_suffix]
rule!(exp, and![(exp_prefix, optional!(exp_suffix)) => ignore]);

// prefixexp_prefix ::= Name | ‘(’ exp ‘)’
rule!(prefixexp_prefix, or![
    variables::Id::rule,
    and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE)) =>
        |stack: &mut stack::Stack| {
            // Remove braces from stack
            let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
            stack.push_single(expression)
        }]]);

// prefixexp_suffix ::= var_suffix [prefixexp_suffix] | functioncall_suffix [prefixexp_suffix]
rule!(prefixexp_suffix, or![
    and![(var_suffix, optional!(prefixexp_suffix)) => ignore],
    and![(functioncall_suffix, optional!(prefixexp_suffix)) => ignore]
]);

// prefixexp ::= prefixexp_prefix [prefixexp_suffix]
rule!(prefixexp, and![(prefixexp_prefix, optional!(prefixexp_suffix)) => ignore]);

// To resolve 3-way recursion (prefixexp, var, functioncall), we need this set of rules
// functioncall_suffix ::= args [functioncall_suffix] | ‘:’ Name args [functioncall_suffix]
rule!(functioncall_suffix, or![
    and![(and![(args) => function::Funcall::new], optional!(functioncall_suffix)) => ignore],
    and![(and![(terminal!(Keyword::COLONS), variables::Id::rule, args) => function::Funcall::new_self], optional!(functioncall_suffix)) => ignore]
]);

// functioncall_repetition ::= functioncall_suffix [functioncall_repetition] | var_suffix [var_suffix] functioncall_suffix [functioncall_repetition]
rule!(functioncall_repetition, or![
    and![(functioncall_suffix, optional!(functioncall_repetition)) => ignore],
    and![(var_suffix, functioncall_suffix, optional!(functioncall_repetition)) => ignore]
]);

// functioncall ::= prefixexp_prefix functioncall_repetition
rule!(functioncall, and![(prefixexp_prefix, functioncall_repetition) => ignore]);

// args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString
rule!(args, or![
    and![(terminal!(Keyword::LBRACE), optional!(explist), terminal!(Keyword::RBRACE)) => function::Funcall::new_args],
    primitives::String::rule
]);

//functiondef ::= function funcbody*/

// funcbody ::= ‘(’ [parlist] ‘)’ block end

// -- Here we have a problem of prefix comma for both variants. Will resolve manually
// -- Names always will produce vector and ellipsis will produce single element, which is the indicator of the end
// See FunctionParameters::new* function for further documentation
// parlist_name ::= Name [parlist_suffix] | ‘...’ [parlist_suffix]
rule!(parlist_name, or![
    and![(and![(variables::Id::rule) => function::FunctionParameters::new_parameter], optional!(parlist_suffix)) => ignore],
    and![(and![(terminal!(Keyword::DOT3)) => function::FunctionParameters::new_final_varargs], optional!(parlist_suffix)) => ignore]
]);

// parlist_suffix ::= ‘,’ parlist_name
rule!(parlist_suffix, and![(terminal!(Keyword::COMMA), parlist_name) => ignore]);

// parlist ::= Name [parlist_suffix] | ‘...’
rule!(parlist, or![
    and![(
        and![(variables::Id::rule) =>
            |stack: &mut stack::Stack| {
                // Each name inside parameters list will produce repetition. Hence we do this with the first name too
                let name = stack.pop_single();

                let mut vec = VecDeque::new();
                vec.push_back(name);
                stack.push_repetition(vec)
            }],
        optional!(parlist_suffix)) => function::FunctionParameters::new_namelist],
    and![(terminal!(Keyword::DOT3)) => function::FunctionParameters::new_varargs]]);

/*

tableconstructor ::= ‘{’ [fieldlist] ‘}’
fieldlist ::= field {fieldsep field} [fieldsep]
field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
fieldsep ::= ‘,’ | ‘;’*/

// binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
//        ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
//        ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
//        and | or
rule!(binop, operators::Binop::rule);
// unop ::= ‘-’ | not | ‘#’ | ‘~’
rule!(unop, operators::Unop::rule);
