use crate::ast::AstNode;



enum Symbol {
    Terminal(String),
    NonTerminal(String),
}

struct Rule {
    symbols: Vec<Symbol>
}

trait Parser {

    fn parse_program(&mut self) -> Result<AstNode, String>;
    fn parse_decl(&mut self) -> Result<AstNode, String>;
    fn parse_expr(&mut self) -> Result<AstNode, String>;
    fn parse_type(&mut self) -> Result<AstNode, String>;
    fn parse_pattern(&mut self) -> Result<AstNode, String>;
    fn parse_atom(&mut self) -> Result<AstNode, String>;
    fn parse_literal(&mut self) -> Result<AstNode, String>; 
}

/*

pub fn parse_expr(&mut self) -> Result<AstNode, String> {
    match self.peek() {
        'if' =>
        'let' =>
        'fn' =>
        _ => {

            
        } 
    } 
}

*/

