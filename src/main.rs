
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
    
    let filepath = &args[1]; 
    let source = read_file(filepath).unwrap();
    let mut lexer = Lexer::new(source);
    let mut head = lexer.tokenize();
    
    while let Some(h) = head {
        println!("{:?}", h.ty);
        head = h.next;
    }
}

