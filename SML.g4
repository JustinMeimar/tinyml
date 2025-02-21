grammar TinyML;

prog : dec* EOF ;

dec : 'val' pat (':' typ)? '=' exp          
    | 'fun' ID match (':' typ)?
    | dec ';' dec                           
    ;

exp : if_exp ;                              

if_exp : let_exp                            
       | 'if' exp 'then' exp 'else' exp
       ;

let_exp : fn_exp                            
        | 'let' dec 'in' exp 'end'
        ;

fn_exp : add_exp                            
       | 'fn' match
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

typ : 'int'                                 
    | 'char'                                
    | 'string'                              
    | VAR                                   
    | typ '->' typ                         
    | typ '*' typ                          
    ;

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
