use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, Display};
use tinyml::util::read_file;

#[derive(PartialEq, Debug, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Keyword {
    Let,
    Fun,
    In,
    End,
    Case,
    Of,
    Val,
    Type,
}

#[derive(PartialEq, Debug, EnumIter, EnumString, Display)]
enum Syntactic {
    #[strum(to_string = "->")]
    Arrow,
    #[strum(to_string = "(")]
    LeftParen,
    #[strum(to_string = ")")]
    RightParen,
    #[strum(to_string = "=")]
    Equal,
    #[strum(to_string = "=>")]
    FatArrow,
    #[strum(to_string = ",")]
    Comma,
    #[strum(to_string = "|")]
    Bar,
}

#[derive(PartialEq, Debug, EnumIter, EnumString)]
enum Operator {
    #[strum(to_string = "+")]
    Plus,
    #[strum(to_string = "-")]
    Minus,
    #[strum(to_string = "*")]
    Multiply,
    #[strum(to_string = "/")]
    Divide,
}

#[derive(PartialEq, Debug, EnumIter, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
enum Constructor {
    None,
    Some,
    Nil,
    Cons,
}

enum TokenType {
    Keyword(Keyword),
    Syntactic(Syntactic),
    Operator(Operator),
    Integer(i32),
    Bool(bool),
    ID(String),
    Constructor(Constructor),
    EOF,
    Error,
}

struct Token {
    line: usize,
    col: usize,
    len: usize,
    ty: TokenType,
    next: Option<Box<Token>>,
}

impl Token {
    pub fn new(line: usize, col: usize, len: usize, ty: TokenType) -> Self {
        return Token {
            line,
            col,
            len,
            ty,
            next: None,
        } 
    }

    pub fn set_next(&mut self, next_token: Token) {
        self.next = Some(Box::new(next_token));
    }
} 

struct Lexer {
    cur_idx: usize,
    start_idx: usize,
    max_idx: usize,
    in_comment: bool,
    pos_line: usize,
    pos_col: usize,
    source: String,
}

impl Lexer {
        
    pub fn new(source: String) -> Self {
        return Lexer {
            cur_idx: 0,
            start_idx: 0,
            max_idx: source.len(),
            pos_line: 0,
            pos_col: 0,
            in_comment: false,
            source,
        };  
    }
    
    pub fn peek(&self) -> Option<char> {
        self.source[self.cur_idx..].chars().nth(1)
    }
    
    pub fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.cur_idx += c.len_utf8();
            self.pos_col += 1;
            Some(c)
        } else {
            None
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        println!("Tokenizing!");
        let mut token_vec = Vec::new(); 
        
        // while we can consume tokens, add them into the vector
        while let Some(scan_token) = self.scan_token() {
            println!("Scanned a token..");
            token_vec.push(scan_token);
        }
        token_vec
    }
    
    fn make_token(&self, len: usize, ty: TokenType) -> Token {
        Token::new(
            self.pos_line,
            self.pos_col,
            len,
            ty
        )
    }

    fn match_token(&self, slice: &str) -> Option<Token> {
        println!("Match token on slice: {}", slice);
        
        // first try and match a keyword
        if let Ok(kw) = Keyword::from_str(slice) {
            return Some(
                self.make_token(
                    slice.len(),
                    TokenType::Keyword(kw)
                )
            );
        } 
       
        // TODO: try to match some syntactic tokens
         
        // TODO: try ID, Integer, Bool etc.

        // TODO: try constructor

        None 
    }

    fn scan_token(&mut self) -> Option<Token> {
        println!("Attempting to scan a token from: start: {} cur: {}", self.start_idx, self.cur_idx);
        let max_len = 10;
        self.skip_ws();
        if self.cur_idx == self.max_idx {
            return Some(Token::new(self.pos_line, self.pos_col, 0, TokenType::EOF));
        }
        
        let mut scanned_token: Option<Token> = None;
        let source_slice = &self.source[self.cur_idx..];
        
        // Try different lengths to find the longest valid token
        for i in 1..max_len {
            if let Some(sub_slice) = source_slice.get(..i) {
                if let Some(token) = self.match_token(sub_slice) {
                    scanned_token = Some(token);
                    println!("Found token of length {}", i);
                }
            }
        }

        // After loop, update position if we found a token
        if let Some(ref token) = scanned_token {
            self.cur_idx += token.len;
            self.pos_col += token.len;
            println!("Scanned!");
        }

        scanned_token
    }
    
    fn skip_ws(&mut self) {
        while self.cur_idx < self.max_idx { 
            let cur_char = self.source[self.cur_idx..]
                .chars()
                .next()
                .unwrap();
            
            if !cur_char.is_whitespace() {
                break;
            } 
            self.cur_idx += cur_char.len_utf8();
        }
    }
}

fn main() {
    println!("Hello, world!");
     
    let source = read_file("./tests/001.ml").unwrap();
    
    // println!("File: {}", source);

    let mut lexer = Lexer::new(source);
    lexer.tokenize();


    for color in Keyword::iter() {
        println!("My favorite color is {:?}", color);
    }

    for syn in Syntactic::iter() {
        println!("My favorite syntax is {:?}", syn.to_string());
    }
    let syn = Syntactic::from_str("->");
    println!("{:?}", syn);

    let kw = Keyword::from_str("let");
    println!("{:?}", kw);
}

