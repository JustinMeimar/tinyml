
// use tinyml::lexer::
use std::env;
use std::process;
use tinyml::util::read_file;
use tinyml::lexer::Lexer;

fn main() {
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
    
    // print token stream
    let mut head = lexer.tokenize(); 
    while let Some(h) = head {
        println!("{:?}", h.ty);
        head = h.next;
    }
}

