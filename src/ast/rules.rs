#![cfg_attr(rustfmt, rustfmt_skip)]

use std::collections::VecDeque;

use crate::ast::parser;
use crate::ast::stack;
use crate::ast::expressions::*;

use crate::ast::lexer::tokens::Keyword;

const DEBUG: bool = false;

fn ignore(_: &mut stack::Stack) -> bool {
    true
}

/// Removes sequences separators (commas, dots, etc).
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

/// Function to remove enclosing brackets
pub fn remove_enclosing_brackets(stack: &mut stack::Stack) {
    let (_rb, expression, _lb) = stack_unpack!(stack, single, single, single);
    stack.push_single(expression);
}

// chunk ::= block
rule!(chunk, block);

// block ::= {stat} [retstat]
rule!(block, and![(repetition!(stat), optional!(retstat, nil)) => blocks::Block::new]);

// stat ::=  ‘;’ |
//      varlist ‘=’ explist |
//      functioncall |
//      label |
//      break |
//      goto Name |
//      do block end |
//      while exp do block end |
//      repeat block until exp |
//      if exp then block {elseif exp then block} [else block] end |
//      for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
//      for namelist in explist do block end |
//      function funcname funcbody |
//      local function Name funcbody |
//      local namelist [‘=’ explist]
rule!(stat, or![
    and![(terminal!(Keyword::SEMICOLONS)) => ignore],
    and![(varlist, terminal!(Keyword::ASSIGN), explist) => variables::Assignment::new],
    functioncall,
    label,
    statements::Statement::breakstat,
    and![(terminal!(Keyword::GOTO), variables::Id::rule) => labels::Goto::new],
    and![(terminal!(Keyword::DO), block, terminal!(Keyword::END)) => blocks::DoBlock::new],
    and![(terminal!(Keyword::WHILE), exp, terminal!(Keyword::DO), block, terminal!(Keyword::END)) => blocks::WhileBlock::new],
    and![(terminal!(Keyword::REPEAT), block, terminal!(Keyword::UNTIL), exp) => blocks::RepeatBlock::new],
    and![(
        terminal!(Keyword::IF),
        exp,
        terminal!(Keyword::THEN),
        block,
        repetition!(and![(
            terminal!(Keyword::ELSEIF),
            exp,
            terminal!(Keyword::THEN),
            block
            ) => blocks::IfCondition::new_elseif]),
        optional!(and![(terminal!(Keyword::ELSE), block) => second], nil),
        terminal!(Keyword::END)) => blocks::IfBlock::new],
    and![(
        terminal!(Keyword::FOR),
        variables::Id::rule,
        or![
            and![(
                terminal!(Keyword::ASSIGN),
                exp,
                terminal!(Keyword::COMMA),
                exp,
                optional!(and![(terminal!(Keyword::COMMA), exp) => second], nil),
                terminal!(Keyword::DO),
                block,
                terminal!(Keyword::END)) => blocks::NumericalForBlock::new],
            and![(
                and![(repetition!(and![(terminal!(Keyword::COMMA), variables::Id::rule) => second])) => prepend_vector_prefix],
                terminal!(Keyword::IN),
                explist,
                terminal!(Keyword::DO),
                block,
                terminal!(Keyword::END)) => blocks::GenericForBlock::new]
        ]) => ignore],
    and![(terminal!(Keyword::FUNCTION), funcname, funcbody) => function::Function::new],
    and![(terminal!(Keyword::LOCAL), or![
            and![(terminal!(Keyword::FUNCTION), variables::Id::rule, funcbody) => function::Function::new],
            and![(namelist, variables::Assignment::rule_local) => ignore]
        ]) => blocks::Local::new]
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
            repetition!(and![(terminal!(Keyword::DOT), variables::Id::rule_string_id) => second])) => prepend_vector_prefix],
        optional!(and![(terminal!(Keyword::COLONS), variables::Id::rule_string_id) => second], nil)) =>
        function::Funcname::new]);

// varlist ::= var {‘,’ var}
// We push vector not expression on top to check assignment parity
rule!(varlist, and![(
    var,
    repetition!(and![(terminal!(Keyword::COMMA), var) => second])) =>
    prepend_vector_prefix]);

// var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
rule!(var_suffix, or![
    and![(and![(terminal!(Keyword::LSBRACKET), exp, terminal!(Keyword::RSBRACKET)) =>
        tables::Indexing::new_table], optional!(var_suffix)) => ignore],
    and![(and![(terminal!(Keyword::DOT), variables::Id::rule_string_id) =>
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
        and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE)) => remove_enclosing_brackets],
        var_repetition) => ignore]]);


// namelist ::= Name {‘,’ Name}
// We push vector not expression on top to check assignment parity
rule!(namelist, and![(
    variables::Id::rule_string_id,
    repetition!(and![(terminal!(Keyword::COMMA), variables::Id::rule_string_id) => second])) =>
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
//        prefixexp | tableconstructor | unop exp
rule!(exp_prefix, or![
    primitives::Nil::rule,
    primitives::Boolean::rule,
    primitives::Number::rule,
    primitives::String::rule,
    statements::Statement::ellipsis,
    functiondef,
    prefixexp,
    tableconstructor,
    unop
]);

// exp ::= binop
rule!(exp, binop);

// prefixexp_prefix ::= Name | ‘(’ exp ‘)’
rule!(prefixexp_prefix, or![
    variables::Id::rule,
    and![(terminal!(Keyword::LBRACE), exp, terminal!(Keyword::RBRACE)) => remove_enclosing_brackets]]);

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
    and![(and![(terminal!(Keyword::COLONS), variables::Id::rule_string_id, args) => function::Funcall::new_self], optional!(functioncall_suffix)) => ignore]
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
    tableconstructor,
    primitives::String::rule
]);

// functiondef ::= function funcbody
rule!(functiondef, and![(terminal!(Keyword::FUNCTION), funcbody) => second]);

// funcbody ::= ‘(’ [parlist] ‘)’ block end
rule!(funcbody, and![(and![(terminal!(Keyword::LBRACE),
                            optional!(parlist, nil),
                            terminal!(Keyword::RBRACE)) =>
                            |stack: &mut stack::Stack| {
                                let (_rb, params, _lb) = stack_unpack!(stack, single, optional, single);
                                stack.push_optional(params);
                            }],
                      block,
                      terminal!(Keyword::END)) => function::Closure::new]);

// -- Here we have a problem of prefix comma for both variants. Will resolve manually
// -- Names always will produce vector and ellipsis will produce single element, which is the indicator of the end
// See FunctionParameters::new* function for further documentation
// parlist_name ::= Name [parlist_suffix] | ‘...’
rule!(parlist_name, or![
    and![(and![(variables::Id::rule_string_id) => function::FunctionParameters::new_name], optional!(parlist_suffix)) => ignore],
    and![(and![(terminal!(Keyword::DOT3)) => function::FunctionParameters::new_namelist_varargs], optional!(parlist_suffix)) => ignore]
]);

// parlist_suffix ::= ‘,’ parlist_name
rule!(parlist_suffix, and![(terminal!(Keyword::COMMA), parlist_name) => ignore]);

// TODO: Rewrite to manual parsing. This should be much cleaner
// parlist ::= Name [parlist_suffix] | ‘...’
rule!(parlist, or![
    and![(
        and![(variables::Id::rule_string_id) =>
            |stack: &mut stack::Stack| {
                // Each name inside parameters list will produce repetition. Hence we do this with the first name too
                let name = stack.pop_single();

                let mut vec = VecDeque::new();
                vec.push_back(name);
                stack.push_repetition(vec)
            }],
        optional!(parlist_suffix)) => function::FunctionParameters::new_namelist],
    and![(terminal!(Keyword::DOT3)) => function::FunctionParameters::new_single_varargs]
]);

// tableconstructor ::= ‘{’ [fieldlist] ‘}’
rule!(tableconstructor, and![(terminal!(Keyword::LCBRACKET), optional!(fieldlist), terminal!(Keyword::RCBRACKET)) => tables::Table::new]);

// fieldlist_suffix ::= fieldsep [fieldlist]
rule!(fieldlist_suffix, and![(fieldsep, optional!(fieldlist)) => ignore]);

// fieldlist ::= field [fieldlist_prefix]
rule!(fieldlist, and![(
    and![(field) => tables::TableField::new_list_name],
    optional!(fieldlist_suffix)) =>
    ignore]);

// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
rule!(field, or![
    and![(terminal!(Keyword::LSBRACKET), exp, terminal!(Keyword::RSBRACKET), terminal!(Keyword::ASSIGN), exp) => tables::TableField::new_table_index],
    and![(tables::TableField::name_rule, terminal!(Keyword::ASSIGN), exp) => tables::TableField::new_object_index],
    and![(exp) => tables::TableField::new_value]
]);

// fieldsep ::= ‘,’ | ‘;’
rule!(fieldsep, or![
    and![((terminal!(Keyword::COMMA))) => |stack: &mut stack::Stack| { stack.pop_single() }],
    and![((terminal!(Keyword::SEMICOLONS))) => |stack: &mut stack::Stack| { stack.pop_single() }]
]);

// binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
//        ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
//        ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
//        and | or
rule!(binop, operators::Binop::rule);

// unop ::= ‘-’ | not | ‘#’ | ‘~’
rule!(unop, operators::Unop::rule);
