grammar VCL;

vcl         : expr EOF ;
expr        : subExpr (conjunction | disjunction | exclusion )? ;
subExpr     : systemUri? (simpleExpr | OPEN expr CLOSE);
conjunction : (COMMA subExpr)+ ;
disjunction : (SEMI subExpr)+ ;
exclusion   : DASH subExpr ;
simpleExpr  : STAR | code | filter | includeVs ;

includeVs   : IN (URI | systemUri) ;
systemUri   : OPEN URI CLOSE;
filter      : (property
                ( EQ code
                | IS_A code
                | IS_NOT_A code
                | DESC_OF code
                | REGEX str
                | IN (codeList | URI | filterList)
                | NOT_IN (codeList | URI | filterList)
                | GENERALIZES code
                | CHILD_OF code
                | DESC_LEAF code
                | EXISTS code     // only true and false are valid codes here
                )
              | (code | codeList | STAR | URI | filterList ) DOT property  // operator "of"
              );
filterList  : LCRLY filter (COMMA filter)* RCRLY ;
property    : code ;

codeList        : LCRLY code (COMMA code)+ RCRLY ;
code            : SCODE | QUOTED_VALUE ;
str             : QUOTED_VALUE ;


DASH          : '-' ;
OPEN          : '(' ;
CLOSE         : ')' ;
LCRLY         : '{' ;
RCRLY         : '}' ;
SEMI          : ';' ;
COMMA         : ',' ;
DOT           : '.' ;
STAR          : '*' ;

EQ            : '=' ;
IS_A          : '<<' ;
IS_NOT_A      : '~<<' ;
DESC_OF       : '<' ;
REGEX         : '/' ;
IN            : '^' ;
NOT_IN        : '~^' ;
GENERALIZES   : '>>' ;
CHILD_OF      : '<!' ;
DESC_LEAF     : '!!<' ;
EXISTS        : '?' ;

URI             : [a-zA-Z]+ [:] [a-zA-Z0-9?=:;&_%+-.@#$^!{}/]+ ('|' ~[|()] *)? ;
SCODE           : [a-zA-Z0-9] [-_a-zA-Z0-9]* ;   // simple codes only
QUOTED_VALUE    : '"' (~["\\] | '\\' ["\\])* '"' ;

WS              : [ \t]+ -> skip ;   // skip spaces, tabs; newlines not permitted