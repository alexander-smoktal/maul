use std::collections::VecDeque;

use ast::parser;
use ast::stack;
use ast::expressions::*;
use ast::lexer::tokens::Keyword;

const DEBUG: bool = false;

fn ignore(_: &mut stack::Stack) -> bool {
    true
}

#[allow(dead_code)]
fn first(stack: &mut stack::Stack) {
    let (_second, first) = stack_unpack!(stack, single, single);
    stack.push_single(first);
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
// Note nexted and!'s. We use internal and! to remove braces and dot
rule!(var_suffix, or![
    and![(
        and![(terminal!(Keyword::LSBRACKET), exp, terminal!(Keyword::RSBRACKET)) =>
            |stack: &mut stack::Stack| {
                // Remove brackets from stack
                let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
                stack.push_single(expression);
                tables::Indexing::new(stack)
            }],
        optional!(var_suffix)) => ignore],
    and![(
        and![(terminal!(Keyword::DOT), variables::Id::rule) =>
            |stack: &mut stack::Stack| {
                // Remove dot from stack
                let (name, _dot) = stack_unpack!(stack, single, single);
                stack.push_single(name);
                tables::Indexing::new(stack)
            }],
        optional!(var_suffix)) => ignore]
]);

// var ::=  Name [var_suffix] | functioncall var_suffix | ‘(’ exp ‘)’ var_suffix !!! no funcall
// Note nexted and!'s. We use internal and! to remove braces
rule!(var, or![
    and![(variables::Id::rule, optional!(var_suffix)) => ignore],
    and![(
        and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE)) =>
            |stack: &mut stack::Stack| {
                // Remove braces from stack
                let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
                stack.push_single(expression)
            }],
        var_suffix) => ignore]]);


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

//prefixexp ::= var | functioncall | ‘(’ exp ‘)’
// -- This one is terrible. To resolve 3-way recursion (prefixexp, var, functioncall), we need this set of rules
// functioncall_suffix1 ::= args [functioncall_suffix1] | ‘:’ Name args [functioncall_suffix1]
rule!(functioncall_suffix1, or![
    and![(
        and![(args) => function::Funcall::new], 
        optional!(functioncall_suffix1)) => ignore],
    and![(
        and![(terminal!(Keyword::COLONS), variables::Id::rule, args) => function::Funcall::new_self], 
        optional!(functioncall_suffix1)) => ignore]
]);

// functioncall_suffix2 ::= var_suffix functioncall_suffix1 [functioncall_suffix2] -- resolved to var
rule!(functioncall_suffix2, and![(var_suffix, functioncall_suffix1, optional!(functioncall_suffix2)) => ignore]);

// functioncall_suffix3 ::= functioncall_suffix1 [functioncall_suffix2]
rule!(functioncall_suffix3, and![(functioncall_suffix1, optional!(functioncall_suffix2)) => ignore]);

// functioncall_suffix4 ::= var_suffix functioncall_suffix3 | functioncall_suffix3 -- either var expression or prefixexp expression
rule!(functioncall_suffix4, or![
    and![(var_suffix, functioncall_suffix3) => ignore],
    functioncall_suffix3
]);

// functioncall ::= Name [var_suffix] functioncall_suffix3 |                   -- var ID
//        ‘(’ exp ‘)’ functioncall_suffix4
rule!(functioncall, or![
    and![(variables::Id::rule, optional!(var_suffix), functioncall_suffix3) => ignore],
    and![(
        and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE)) =>
            |stack: &mut stack::Stack| {
                // Remove braces from stack
                let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
                stack.push_single(expression)
            }],
        functioncall_suffix4) => ignore]
]);

// args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString
rule!(args, or![
    and![(terminal!(Keyword::LBRACE), optional!(explist), terminal!(Keyword::RBRACE)) => function::Funcall::new_args],
    primitives::String::rule
]);

/*functiondef ::= function funcbody
funcbody ::= ‘(’ [parlist] ‘)’ block end
parlist ::= namelist [‘,’ ‘...’] | ‘...’
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
