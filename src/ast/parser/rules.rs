use std::ops;

use ast::parser;
use ast::expressions::{primitives, statements, expression, operators, utils};
use ast::lexer::tokens::Keyword;

const DEBUG: bool = false;

/*chunk ::= block
block ::= {stat} [retstat]
stat ::=  ‘;’ | 
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
// retstat ::= return [explist] [‘;’]
rule!(retstat, or![
    and![(terminal!(Keyword::RETURN), exp, terminal!(Keyword::SEMICOLONS)) => 
        |_ret, exp, _semi| utils::some_expression(statements::Statement::Return(exp))],
    and![(terminal!(Keyword::RETURN), exp) => 
        |_ret, exp| utils::some_expression(statements::Statement::Return(exp))],
    terminal!(Keyword::RETURN)
]);
/*
label ::= ‘::’ Name ‘::’
funcname ::= Name {‘.’ Name} [‘:’ Name]
varlist ::= var {‘,’ var}
var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
var ::=  Name [opt_var_suffix] | functioncall var_suffix | ‘(’ exp ‘)’ var_suffix
namelist ::= Name {‘,’ Name}*/
// explist ::= exp {‘,’ exp}
rule!(explist, and![(exp, optional!(terminal!(Keyword::COMMA)), optional!(explist)) => expression::Expressions::new]);

// exp_suffix ::= binop [exp_suffix]
rule!(exp_suffix, and![(binop, optional!(exp_suffix)) => |_, _| None]);


// exp_prefix ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef | 
//        prefixexp | tableconstructor | unop exp
rule!(exp_prefix, or![
    primitives::Nil::rule,
    primitives::Boolean::rule,
    primitives::Number::rule,
    primitives::String::rule,
    statements::Statement::ellipsis
]);
// exp ::= exp_prefix [exp_suffix]
rule!(exp, and![(exp_prefix, optional!(exp_suffix)) => |pref, _| Some(pref)]);


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
rule!(binop, or![terminal!(Keyword::PLUS), terminal!(Keyword::MINUS), terminal!(Keyword::MUL), terminal!(Keyword::DIV), 
    terminal!(Keyword::PATH), terminal!(Keyword::POW), terminal!(Keyword::MOD), terminal!(Keyword::AND), terminal!(Keyword::TILDA),
    terminal!(Keyword::OR), terminal!(Keyword::SHRIGHT), terminal!(Keyword::SHLEFT), terminal!(Keyword::DOT2), terminal!(Keyword::LESS),
    terminal!(Keyword::LEQ), terminal!(Keyword::GREATER), terminal!(Keyword::GEQ), terminal!(Keyword::EQ), terminal!(Keyword::NEQ),
    terminal!(Keyword::AND), terminal!(Keyword::OR)]);
// unop ::= ‘-’ | not | ‘#’ | ‘~’*/
rule!(unop, or![terminal!(Keyword::MINUS), terminal!(Keyword::NOT), terminal!(Keyword::HASH), terminal!(Keyword::TILDA)]);
