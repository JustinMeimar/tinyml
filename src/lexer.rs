use std::{boxed, fs::File};
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, Display};

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString)]
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

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString, Display)]
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

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString)]
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

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
enum Constructor {
    None,
    Some,
    Nil,
    Cons,
}

#[derive(Eq, PartialEq, Debug)]
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
        // Define regex patterns in data section
        lazy_static::lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]*$").unwrap();
            static ref INT_RE: Regex = Regex::new(r"^[0-9_]+$").unwrap();
            static ref FLOAT_RE: Regex = Regex::new(r"^[0-9]+\.[0-9]+$").unwrap();
            static ref BOOL_RE: Regex = Regex::new(r"^(true|false)$").unwrap();
        }

        // Match against patterns in a specific order
        if BOOL_RE.is_match(s) {
            Ok(Literal::Bool(s.to_string()))
        } else if FLOAT_RE.is_match(s) {
            Ok(Literal::Float(s.to_string()))
        } else if INT_RE.is_match(s) {
            Ok(Literal::Integer(s.to_string()))
        } else if ID_RE.is_match(s) {
            Ok(Literal::Id(s.to_string()))
        } else {
            Err(ParseLiteralError(s.to_string()))
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
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

    fn match_token_type(&self, slice: &str) -> Option<TokenType> {
        // Try and match a keyword
        if let Ok(kw) = Keyword::from_str(slice) {
            return Some(TokenType::Keyword(kw));
        } 
       
        // Try to match some syntactic tokens
        else if let Ok(syn) = Syntactic::from_str(slice) {
            return Some(TokenType::Syntactic(syn));
        }
 
        // Try to match some syntactic tokens
        else if let Ok(op) = Operator::from_str(slice) {
            return Some(TokenType::Operator(op));
        }

        // Try to match a cons
        else if let Ok(cons) = Constructor::from_str(slice) {
            return Some(TokenType::Constructor(cons));
        }
        
        // Try to match a cons
        else if let Ok(lit) = Literal::from_str(slice) {
            return Some(TokenType::Literal(lit));
        }
        None 
    }
    
    fn scan_token(&mut self) -> Option<Token> {
        // 
        self.start_idx = self.cur_idx; 
        self.skip_ws();
        if self.cur_idx >= self.max_idx {
            return Some(Token::new(self.pos_line, self.pos_col, 0, TokenType::EOF));
        }
        
        let source_slice = &self.source[self.cur_idx..]; 
        let end_pos = source_slice.find(|c: char| c.is_whitespace())
            .unwrap_or(source_slice.len());
        
        // Try to match the whole slice up to whitespace as a fast path
        let whole_slice = &source_slice[..end_pos];
        if let Some(token_type) = self.match_token_type(whole_slice) {
            let token = self.make_token(whole_slice.len(), token_type);
            self.cur_idx += whole_slice.len();
            self.pos_col += whole_slice.len();
            return Some(token);
        }
        
        let max_len = std::cmp::min(10, source_slice.len());
        let mut longest_token: Option<(usize, TokenType)> = None; 
        for i in 1..=max_len {
            if let Some(sub_slice) = source_slice.get(..i) {
                if let Some(token_type) = self.match_token_type(sub_slice) {
                    longest_token = Some((sub_slice.len(), token_type));
                }
            }
        }
        
        // After finding longest valid token, create and return it
        if let Some((len, token_type)) = longest_token {
            let token = self.make_token(len, token_type);
            self.cur_idx += len;
            self.pos_col += len;
            return Some(token);
        }
        None
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

