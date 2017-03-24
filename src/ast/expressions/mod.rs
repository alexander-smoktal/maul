mod function;
mod id;

use std::vec::Vec;

use ast::parser;

pub enum Expression {
    Function(function::Function),
    Id(id::Id),
}

pub type Expressions = Vec<Expression>;

// chunk ::= block

// block ::= {stat} [retstat]

// stat ::=  ‘;’ |
//     varlist ‘=’ explist |
//     functioncall |
//     label |
//     break |
//     goto Name |
//     do block end |
//     while exp do block end |
//     repeat block until exp |
//     if exp then block {elseif exp then block} [else block] end |
//     for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
//     for namelist in explist do block end |
//     function funcname funcbody |
//     local function Name funcbody |
//     local namelist [‘=’ explist]

impl Expression {
    pub fn from_parser(parser: &parser::Parser) -> Option<Expression> {
        //let lexer_copy = lexer.clone();

        None
    }
}