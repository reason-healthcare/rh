grammar FFQ;

// =======================
// Entry points
// =======================

start
  : header* query EOF
  ;

header
  : AT ALIAS WS? alias '=' WS? systemTarget
  ;

systemTarget
  : systemRef            // e.g., @alias sct = http://snomed.info/sct|20250731
  | valueSetRef          // e.g., @alias dm = vs(https://.../ValueSet/diabetes)
  ;

// Top-level expression over CLAUSES with boolean ops
query
  : expr
  ;

// Precedence: ! > & > | > -
expr
  : expr MINUS expr                      # MinusExpr
  | expr OR expr                         # OrExpr
  | expr AND expr                        # AndExpr
  | NOT expr                             # NotExpr
  | clause                               # ClauseAtom
  | LPAREN expr RPAREN                   # ParenExpr
  ;

// A clause scopes an expression to a CodeSystem (optionally versioned)
clause
  : systemRef (PIPE version=CODE)? COLON innerExpr
  ;

// Inner expr (within a clause)
innerExpr
  : innerExpr MINUS innerExpr            # InnerMinus
  | innerExpr OR innerExpr               # InnerOr
  | innerExpr AND innerExpr              # InnerAnd
  | NOT innerExpr                        # InnerNot
  | primary                              # InnerPrimary
  ;

primary
  : LPAREN innerExpr RPAREN
  | hierarchy
  | propertyExpr
  | membership
  | existsExpr
  ;

// =======================
// Terms
// =======================

// Hierarchy (subsumption) operators
hierarchy
  : (LT2 | LT | ISA) WS? codeRef
  ;

// Property comparisons: = , in(...) , ~ /regex/
propertyExpr
  : propRef WS? '=' WS? value
  | propRef WS? IN WS? LPAREN valueList RPAREN
  | propRef WS? REGEX_OP WS? regex
  ;

// Membership in another ValueSet (by URL or alias)
membership
  : IN WS? ( valueSetRef | HASH alias )
  ;

// Existence of a property
existsExpr
  : HAS WS propRef
  ;

// =======================
// Helpers
// =======================

valueList
  : value (WS? COMMA WS? value)*
  ;

value
  : STRING
  | CODE
  | uri
  ;

codeRef
  : CODE
  | STRING
  ;

propRef
  : dottedName
  ;

dottedName
  : IDENT (DOT IDENT)*
  ;

// CodeSystem ref can be a URI (optionally versioned via PIPE before ':') or an alias
systemRef
  : uri
  | alias
  ;

valueSetRef
  : VS LPAREN uri RPAREN
  ;

uri
  : URI
  ;

alias
  : IDENT
  ;

// =======================
// LEXER
// =======================

AT        : '@';
ALIAS     : 'alias';

AND       : '&';
OR        : '|';
MINUS     : '-';
NOT       : '!';
COLON     : ':';
LPAREN    : '(';
RPAREN    : ')';
COMMA     : ',';
DOT       : '.';
HASH      : '#';
PIPE      : '|';

IN        : 'in';
ISA       : 'isa';
HAS       : 'has';
VS        : 'vs';
REGEX_OP  : '~';

fragment DIGIT : [0-9];
fragment ALPHA : [A-Za-z];
fragment ALNUM : [A-Za-z0-9];
fragment UNRES : [A-Za-z0-9_~\\-\\.]; // unreserved-ish

// Very permissive URI, excludes whitespace, '|', parentheses, commas, '#'
URI
  : ( 'http' 's'? '://' | 'urn:' )
    (~[ \t\r\n|(),#])+                // stop before operators/separators
  ;

// Concept codes / version strings (digits + common punct)
CODE
  : [A-Za-z0-9][A-Za-z0-9._:-]*
  ;

IDENT
  : [A-Za-z_][A-Za-z0-9_]*
  ;

STRING
  : '"' ( '\\' . | ~["\\\r\n] )* '"'
  ;

// Regex literal like /foo.*/ with escaped slashes allowed
REGEX
  : '/' ( '\\/' | ~[/\r\n] )+ '/'
  ;

// Whitespace & comments
WS
  : [ \t\r\n]+ -> skip
  ;

LINE_COMMENT
  : '//' ~[\r\n]* -> skip
  ;

BLOCK_COMMENT
  : '/*' .*? '*/' -> skip
  ;

