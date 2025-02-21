use std::{boxed, fs::File};
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, Display};

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString, Display, Clone)]
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

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString, Display, Clone)]
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

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString, Display, Clone)]
enum Operator {
    #[strum(to_string = "+")]
    Plus,
    #[strum(to_string = "-")]
    Minus,
    #[strum(to_string = "*")]
    Multiply,
    #[strum(to_string = "/")]
    Divide,
    #[strum(to_string = "<")]
    StrictLess,
    #[strum(to_string = "<=")]
    Less,
    #[strum(to_string = ">")]
    StrictGreater,
    #[strum(to_string = ">=")]
    Greater
}

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString, Display, Clone)]
#[strum(serialize_all = "UPPERCASE")]
enum Constructor {
    None,
    Some,
    Nil,
    Cons,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Literal {
    Id(String),
    Bool(String),
    Integer(String),
    Float(String),
}

#[derive(Debug)]
struct ParseLiteralError(String);

impl std::fmt::Display for ParseLiteralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse literal: {}", self.0)
    }
}

impl std::error::Error for ParseLiteralError {}

impl FromStr for Literal {
    type Err = ParseLiteralError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Define regex patterns to match just at the beginning
        lazy_static::lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]*").unwrap();
            static ref INT_RE: Regex = Regex::new(r"^[0-9_]+").unwrap();
            static ref FLOAT_RE: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
            static ref BOOL_RE: Regex = Regex::new(r"^(true|false)").unwrap();
        }

        // Use find() to get the matched text and its length
        if let Some(m) = BOOL_RE.find(s) {
            Ok(Literal::Bool(m.as_str().to_string()))
        } else if let Some(m) = FLOAT_RE.find(s) {
            Ok(Literal::Float(m.as_str().to_string()))
        } else if let Some(m) = INT_RE.find(s) {
            Ok(Literal::Integer(m.as_str().to_string()))
        } else if let Some(m) = ID_RE.find(s) {
            Ok(Literal::Id(m.as_str().to_string()))
        } else {
            Err(ParseLiteralError(s.to_string()))
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    Keyword(Keyword),
    Syntactic(Syntactic),
    Operator(Operator), 
    Constructor(Constructor),
    Literal(Literal),
    EOF,
    Error,
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub col: usize,
    pub len: usize,
    pub ty: TokenType,
    pub next: Option<Box<Token>>,
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

pub struct Lexer {
    cur_idx: usize,
    start_idx: usize,
    max_idx: usize,
    in_comment: bool,
    pos_line: usize,
    pos_col: usize,
    source: String,
    pub token_dict: HashMap<String, TokenType>
}

use std::collections::HashMap;
impl Lexer {
    fn make_token_dict() -> HashMap<String, TokenType> {
        let mut token_map = HashMap::new();
        for kw in Keyword::iter() {
            token_map.insert(format!("{:?}", kw).to_lowercase(), TokenType::Keyword(kw));
        }
        for op in Operator::iter() {
            token_map.insert(op.to_string(), TokenType::Operator(op));
        }
        for syn in Syntactic::iter() {
            token_map.insert(syn.to_string(), TokenType::Syntactic(syn));
        }
        for cons in Constructor::iter() {
            token_map.insert(cons.to_string().to_uppercase(), TokenType::Constructor(cons));
        }
        token_map
    }
    
    pub fn new(source: String) -> Self {
        return Lexer {
            cur_idx: 0,
            start_idx: 0,
            max_idx: source.len(),
            pos_line: 0,
            pos_col: 0,
            in_comment: false,
            source,
            token_dict: Lexer::make_token_dict()
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
    
    pub fn tokenize(&mut self) -> Option<Box<Token>> {  
        let mut head: Option<Box<Token>> = None;
        let mut cur = &mut head;

        while let Some(token) = self.scan_token() {
            // move token onto the heap     
            *cur = Some(Box::new(token));
             
            // check the heap allocation worked and increment cur to next
            if let Some(boxed_token) = cur {
                // check if we reached EOF token
                if boxed_token.ty == TokenType::EOF {
                    break;
                }
                cur = &mut boxed_token.next;
            } 
        }
        head
    }

    fn make_token(&self, len: usize, ty: TokenType) -> Token {
        Token::new(
            self.pos_line,
            self.pos_col,
            len,
            ty
        )
    } 
   
    fn scan_token(&mut self) -> Option<Token> {
        self.skip_ws();
        self.start_idx = self.cur_idx;
        
        if self.cur_idx >= self.source.len() {
            return Some(Token::new(self.pos_line, self.pos_col, 0, TokenType::EOF));
        }
        
        let source_slice = &self.source[self.start_idx..];

        // Look for longest statically defined tokens 
        let longest_match = self.token_dict.iter()
            .filter(|(tok_key, _)| source_slice.starts_with(&**tok_key))
            .max_by_key(|(tok_key, _)| tok_key.len())
            .map(|(tok_key, tok_ty)| (tok_ty.clone(), tok_key.len()));


        match longest_match {
            Some((tok_ty, tok_len)) => { 
                // Advance the lexer position
                self.cur_idx += tok_len;
                self.pos_col += tok_len;
                Some(self.make_token(tok_len, tok_ty))
            },
            None => {
                // Try literal patterns
                match Literal::from_str(source_slice) {
                    Ok(lit) => {
                        // Get the length of the match
                        let lit_len = match &lit {
                            Literal::Id(s) => s.len(),
                            Literal::Bool(s) => s.len(),
                            Literal::Integer(s) => s.len(),
                            Literal::Float(s) => s.len(),
                        }; 
                        // Advance the lexer position
                        self.cur_idx += lit_len;
                        self.pos_col += lit_len;
                        Some(self.make_token(lit_len, TokenType::Literal(lit)))
                    },
                    Err(_) => {
                        // Invalid token
                        self.cur_idx += 1;
                        self.pos_col += 1;
                        Some(self.make_token(1, TokenType::Error))
                    }
                }
            }
        }
    }

    fn skip_ws(&mut self) {
        while self.cur_idx < self.max_idx { 
            let cur_char = self.source[self.cur_idx..]
                .chars()
                .next()
                .unwrap();
            
            if cur_char == '\n' {
                self.pos_line += 1;
                self.pos_col = 0;
            }
            if !cur_char.is_whitespace() {
                break;
            } 
            self.cur_idx += cur_char.len_utf8();
        }
    }
}

