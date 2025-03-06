grammar TinyML;

prog : dec* EOF ;

dec : 'val' pat (':' typ)? '=' exp          
    | 'fun' ID match (':' typ)?
    | dec ';' dec                           
    ;


exp : 'if' exp 'then' exp 'else' exp
    | 'let' dec 'in' exp 'end'
    | 'fn' match
    |  add_exp
    ;

add_exp : mul_exp                           
        | add_exp ('+' | '-') mul_exp
        ;

mul_exp : app_exp                           
        | mul_exp ('*' | '/') app_exp
        ;

app_exp : atom                              
        | app_exp atom
        ;

atom : literal                              
     | ID                                   
     | '(' ')'                              
     | '(' exp ')'                          
     | '(' exp ',' exp (',' exp)* ')'       
     | '[' ']'                              
     | '[' exp (',' exp)* ']'               
     ;

pat : literal                               
    | '_'                                   
    | ID                                    
    | '(' pat ',' pat ')'                   
    ;

match : pat '=>' exp                        
      | pat '=>' exp '|' match              
      ;


type : 'int' typ_rest
    | 'char' typ_rest
    | 'string' typ_rest
    | var typ_rest

typ_rest : '->' type typ_rest
        | '*' type typ_rest
        | ε

var: ''' ID;

literal : INT | CHAR | STRING | BOOL ;

BOOL : 'true' | 'false' ;
INT : '~'? NUM ;
CHAR : '#"' ASCII '"' ;
STRING : '"' ASCII* '"' ;
ID : LETTER (LETTER | DIGIT | '\'' | '_')* ;
VAR : '\'' (LETTER | DIGIT | '\'' | '_')* ;

op : '+' | '-' | '*' | '/' | '=' | '<' | '>' | '<=' | '>=' ;

fragment LETTER : [a-zA-Z] ;
fragment DIGIT : [0-9] ;
fragment NUM : DIGIT+ ;
fragment ASCII : ~["\r\n] ;

WS : [ \t\r\n]+ -> skip ;
COMMENT : '(*' .*? '*)' -> skip ;
