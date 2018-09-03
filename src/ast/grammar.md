
LL(1) Parser grammar
======
Original LUA 5.3 reference grammar
----
Because LUA grammar from the reference manual contains left recursion it doesn't fit out needs.
The grammar rules should be reworked to be DFA and not contain left recursion.
The original grammar:
```lua
chunk ::= block
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
        local namelist [‘=’ explist] 
retstat ::= return [explist] [‘;’]
label ::= ‘::’ Name ‘::’
funcname ::= Name {‘.’ Name} [‘:’ Name]
varlist ::= var {‘,’ var}
var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name 
namelist ::= Name {‘,’ Name}
explist ::= exp {‘,’ exp}
exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef | 
        prefixexp | tableconstructor | exp binop exp | unop exp 
prefixexp ::= var | functioncall | ‘(’ exp ‘)’
functioncall ::=  prefixexp args | prefixexp ‘:’ Name args 
args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString 
functiondef ::= function funcbody
funcbody ::= ‘(’ [parlist] ‘)’ block end
parlist ::= namelist [‘,’ ‘...’] | ‘...’
tableconstructor ::= ‘{’ [fieldlist] ‘}’
fieldlist ::= field {fieldsep field} [fieldsep]
field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
fieldsep ::= ‘,’ | ‘;’
binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ | 
        ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ | 
        ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ | 
        and | or
unop ::= ‘-’ | not | ‘#’ | ‘~’
```

Modified LUA 5.3 LL(1) grammar
----
Modified grammar. Without left recursion (including indirect) and rules to be suitable for parsing with LL(1) parser.
```lua
chunk ::= block
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
        local namelist [‘=’ explist] 
retstat ::= return [explist] [‘;’]
label ::= ‘::’ Name ‘::’
funcname ::= Name {‘.’ Name} [‘:’ Name]
varlist ::= var {‘,’ var}
var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
var ::=  Name [opt_var_suffix] | functioncall var_suffix | ‘(’ exp ‘)’ var_suffix
namelist ::= Name {‘,’ Name}
explist ::= exp {‘,’ exp}
exp_suffix ::= binop exp [exp_suffix]
exp_prefix ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef | 
        prefixexp | tableconstructor | unop exp
exp ::= exp_prefix [exp_suffix]
prefixexp ::= var | functioncall | ‘(’ exp ‘)’
-- This one is terrible. To resolve 3-way recursion (prefixexp, var, functioncall), we need this set of rules
functioncall_suffix1 ::= args [functioncall_suffix1] | ‘:’ Name args [functioncall_suffix1]
functioncall_suffix2 ::= var_suffix functioncall_suffix1 [functioncall_suffix2] -- resolved to var
functioncall_suffix3 ::= functioncall_suffix1 [functioncall_suffix2]
functioncall_suffix4 ::= var_suffix functioncall_suffix3 | functioncall_suffix3 -- either var expression or prefixexp expression
functioncall ::= Name [var_suffix] functioncall_suffix3 |                   -- var ID
        ‘(’ exp ‘)’ functioncall_suffix4
args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString 
functiondef ::= function funcbody
funcbody ::= ‘(’ [parlist] ‘)’ block end
parlist ::= namelist [‘,’ ‘...’] | ‘...’
tableconstructor ::= ‘{’ [fieldlist] ‘}’
fieldlist ::= field {fieldsep field} [fieldsep]
field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
fieldsep ::= ‘,’ | ‘;’
binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ | 
        ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ | 
        ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ | 
        and | or
unop ::= ‘-’ | not | ‘#’ | ‘~’
```