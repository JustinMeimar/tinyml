
use std::env;
use std::process;
use std::error::Error;
use tinyml::ast_visitor::Visitable;
use tinyml::util::read_file;
use tinyml::lexer::Lexer;
use tinyml::parser::Parser;
use tinyml::passes::{
    visit_def::DefVisitor,
    visit_debug::DebugVisitor
}; 


fn run(source: String) -> Result<(), Box<dyn Error>> {
    
    // create the lexer
    let mut lexer = Lexer::new(source); 
    
    // get token stream
    let tokens = lexer.tokenize();
    
    // move tokens into the parser
    let mut parser = Parser::new(tokens); 
    
    // parse and return the parse tree
    let ast = parser.parse()?;
    
    // build the AST visitor
    let mut debug_visitor = DebugVisitor::new();
    
    // visit the ast
    debug_visitor.visit(&*ast)?;
    
    // create a visitor to define symbols 
    let mut def_visitor = DefVisitor::new();
    def_visitor.visit(&*ast)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filepath>", args[0]);
        process::exit(1);
    }
    
    // read source file
    let filepath = &args[1];
    let source = read_file(filepath).unwrap();
    
    if let Err(e) = run(source) {
        let display_error = format!("{}", e); // not sure how else to call display
        return Err(display_error.into())
    } 

    Ok(())
}

