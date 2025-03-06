
use std::env;
use std::process;
use std::error::Error;
use tinyml::util::read_file;
use tinyml::lexer::Lexer;
use tinyml::parser::Parser;
use tinyml::ast_visitor::DebugVisitor;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filepath>", args[0]);
        process::exit(1);
    }
    
    // read source file
    let filepath = &args[1];
    let source = read_file(filepath).unwrap();
    
    // create the lexer
    let mut lexer = Lexer::new(source); 
    
    // get token stream
    let tokens = lexer.tokenize();
    
    // move tokens into the parser
    let mut parser = Parser::new(tokens); 
    
    // parse and return the parse tree
    let ast = parser.parse()?;
    
    // build the AST visitor
    let mut visitor = DebugVisitor::new();
    
    // visit the ast
    visitor.visit::<()>(&*ast)?;
    
    Ok(())
}

