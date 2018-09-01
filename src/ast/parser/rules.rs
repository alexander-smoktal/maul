use ast::parser;
use ast::stack;
use ast::expressions::{primitives, statements, expression, operators, utils, labels, variables, tables};
use ast::lexer::tokens::Keyword;

const DEBUG: bool = false;

fn ignore(_: &mut stack::Stack) -> bool {
    true
}

#[allow(dead_code)]
fn first(stack: &mut stack::Stack) -> bool {
    let (_second, first) = stack_unpack!(stack, single, single);
    stack.push_single(first);
    true
}

fn second(stack: &mut stack::Stack) -> bool {
    let (second, _first) = stack_unpack!(stack, single, single);
    stack.push_single(second);
    true
}

/*chunk ::= block
block ::= {stat} [retstat]*/


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
        local namelist [‘=’ explist] */

// retstat ::= return [explist] [‘;’]
/*rule!(retstat, and![(terminal!(Keyword::RETURN), optional!(explist), optional!(terminal!(Keyword::SEMICOLONS))) =>
                    |_, explist, _| utils::some_expression(statements::Statement::Return(explist))]);
// label ::= ‘::’ Name ‘::’
rule!(name, and![(terminal!(Keyword::PATH), variables::Id::name, terminal!(Keyword::PATH)) =>
                 |_, name, _| utils::some_expression(labels::Label(name))]);
/*funcname ::= Name {‘.’ Name} [‘:’ Name]*/
// varlist ::= var {‘,’ var}
rule!(varlist, and![(var, optional!(terminal!(Keyword::COMMA)), optional!(varlist)) => variables::Varlist::new]);

// var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
rule!(var_suffix, or![
    and![(terminal!(Keyword::LSBRACKET), exp, terminal!(Keyword::RSBRACKET), optional!(var_suffix)) =>
        |_, expr, _, suffix| utils::some_expression(expression::Expressions {
            head: Box::new(tables::Indexing(expr)),
            tail: suffix
        })],
    and![(terminal!(Keyword::DOT), variables::Id::name, optional!(var_suffix)) =>
        |_, id, suffix| utils::some_expression(expression::Expressions {
            head: Box::new(tables::Indexing(id)),
            tail: suffix
        })]
]);

// var ::=  Name [var_suffix] | functioncall var_suffix | ‘(’ exp ‘)’ var_suffix !!! no funcall
rule!(var, or![
    and![(variables::Id::name, optional!(var_suffix)) =>
        |head, tail| utils::some_expression(expression::Expressions {
            head,
            tail
        })],
    and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE), var_suffix) =>
        |_, head, _, tail| utils::some_expression(expression::Expressions {
            head,
            tail: Some(tail)
        })]
]);

// namelist ::= Name {‘,’ Name}
rule!(namelist, and![(variables::Id::name, optional!(terminal!(Keyword::COMMA)), optional!(variables::Id::name)) => variables::Varlist::new]);
*/
// explist ::= exp {‘,’ exp}
rule!(explist, and![(
    exp, 
    repetition!(and![(
        terminal!(Keyword::COMMA),
        exp) =>
        second])) => 
    expression::Expressions::new]);

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

/*
prefixexp ::= var | functioncall | ‘(’ exp ‘)’
-- This one is terrible. To resolve 3-way recursion (prefixexp, var, functioncall), we need this set of rules
functioncall_suffix1 ::= args [functioncall_suffix1] | ‘:’ Name args [functioncall_suffix1]
functioncall_suffix2 ::= var_suffix functioncall_suffix1 [functioncall_suffix2] -- resolved to var
functioncall_suffix3 ::= functioncall_suffix1 [functioncall_suffix2]
functioncall_suffix4 ::= var_suffix functioncall_suffix3 | functioncall_suffix3 -- either var expression or prefixexp expression
functioncall ::= Name [opt_var_suffix] functioncall_suffix3 |                   -- var ID
        ‘(’ exp ‘)’ functioncall_suffix4
args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString
functiondef ::= function funcbody
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
