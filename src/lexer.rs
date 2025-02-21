use std::{boxed, fs::File};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, Display};

#[derive(Eq, PartialEq, Debug, EnumIter, EnumString, Display, Clone)]
pub enum TokenType {
    // Keywords
    #[strum(to_string = "let")]
    Let,
    #[strum(to_string = "fun")]
    Fun,
    #[strum(to_string = "in")]
    In,
    #[strum(to_string = "end")]
    End,
    #[strum(to_string = "case")]
    Case,
    #[strum(to_string = "of")]
    Of,
    #[strum(to_string = "val")]
    Val,
    #[strum(to_string = "type")]
    Type,
    #[strum(to_string = "nil")]
    Nil,
    #[strum(to_string = "none")]
    None,
    #[strum(to_string = "some")]
    Some,
    #[strum(to_string = "int")]
    TypeInt,
    #[strum(to_string = "bool")]
    TypeBool,
    #[strum(to_string = "'a")]
    TypeAlpha,
    #[strum(to_string = "if")]
    If,
    #[strum(to_string = "then")]
    Then,
    #[strum(to_string = "else")]
    Else,

    // Syntactic elements
    Comment,
    #[strum(to_string = "(*")]
    OpenComment,
    #[strum(to_string = "*)")]
    CloseComment,
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
    #[strum(to_string = "::")]
    Cons,
    #[strum(to_string = ":")]
    Colon,
    #[strum(to_string = "_")]
    Wildcard,

    // Operators
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
    Greater,
    
    Id(String),
    Bool(String),
    Integer(String),
    Float(String),
    EOF,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
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
} 

pub struct Lexer {
    cur_idx: usize,
    start_idx: usize,
    max_idx: usize,
    comment_depth: usize,
    pos_line: usize,
    pos_col: usize,
    source: String,
    pub token_dict: TokenDict
}

type SourceIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub struct TokenDict {
    pub map: HashMap<String, TokenType>,
    pub max_key_len: usize
}

impl TokenDict {

    pub fn new() -> Self {
        
        let mut map = HashMap::new();
        let mut max_key_len = 0;
        for token_type in TokenType::iter() {
            match token_type {
                // These token types do not belong in the token map, since they are
                // handled specially.
                TokenType::Id(_) | TokenType::Integer(_)
                                 | TokenType::Bool(_) 
                                 | TokenType::Float(_) 
                                 | TokenType::Error
                                 | TokenType::EOF  => continue,
                _ => {}
                             
            }
            let token_string = token_type.to_string();
            if token_string.len() < max_key_len {
                max_key_len = token_string.len();
            }
            map.insert(token_string, token_type);
        }

        map.insert(String::from("true"), TokenType::Bool(String::from("true")));
        map.insert(String::from("false"), TokenType::Bool(String::from("false")));
 
        TokenDict {
            map,
            max_key_len,
        }
    }
}

impl Lexer {
 
    pub fn new(source: String) -> Self {
        return Lexer {
            cur_idx: 0,
            start_idx: 0,
            max_idx: source.len(),
            pos_line: 1,
            pos_col: 1,
            comment_depth: 0,
            source,
            token_dict: TokenDict::new()
        };  
    }
  
    pub fn match_id_or_kw(&self, chars: &str)
        -> (Option<TokenType>, usize)
    {
        let mut len = 0;
        let mut id_or_kw = String::new(); 
        for c in chars.chars() {
            if c.is_alphanumeric() || c == '_' {
                id_or_kw.push(c);
                len += 1;
            } else {
                break;
            }
        } 
        match self.token_dict.map.get(&id_or_kw) {
            Some(token_ty) => {
                (Some(token_ty.clone()), len)
            },
            None => {
                (Some(TokenType::Id(id_or_kw)), len)
            }
        }
    }

    pub fn match_number(&self, chars: &str)
        -> (Option<TokenType>, usize)
    {
        let mut len = 0;
        let mut number = String::new();
        let mut is_float = false;
        
        // Collect integer part
        for c in chars.chars() {
            if c.is_digit(10) {
                number.push(c);
                len += 1;
            } else if c == '.' && !is_float {
                is_float = true;
                number.push(c);
                len += 1;
            } else if is_float && c.is_digit(10) {
                number.push(c);
                len += 1;
            } else {
                break;
            }
        } 
        if number.is_empty() {
            return (None, 0);
        } 
        if is_float {
            (Some(TokenType::Float(number)), len)
        } else {
            (Some(TokenType::Integer(number)), len)
        }
    }

    pub fn match_syntax(&self, chars: &str)
        -> (Option<TokenType>, usize)
    {    
        // Try to match a two-character token first
        let mut chars_iter = chars.chars();
        let first = chars_iter.next();
        let second = chars_iter.next();
        
        // Try the two-character token first
        if let (Some(first_char), Some(second_char)) = (first, second) {
            let two_char_token = format!("{}{}", first_char, second_char);
            
            if let Some(tt) = self.token_dict.map.get(&two_char_token) {
                return (Some(tt.clone()), 2);
            }
        } 
        // If two-character match fails, try single character
        if let Some(first_char) = first {
            let string_value = first_char.to_string();
            if let Some(tt) = self.token_dict.map.get(&string_value) {
                return (Some(tt.clone()), 1);
            }
        }
        (None, 0)
    }

    // pub fn tokenize(&mut self) -> Option<Box<Token>> {
    pub fn tokenize(&mut self) -> Vec<Token> {
        // let mut head: Option<Box<Token>> = None;
        let mut tokens: Vec<Token> = Vec::new(); 
        // let mut cur = &mut head;
        
        while self.cur_idx < self.source.len() {
            self.start_idx = self.cur_idx;
            
            // Get the remaining part of the source
            let remaining = &self.source[self.cur_idx..];
            
            // Skip whitespaces
            if let Some(ch) = remaining.chars().next() {
                if ch.is_whitespace() {
                    self.cur_idx += 1;
                    self.pos_col += 1; 
                    if ch == '\n' {
                        self.pos_line += 1;
                        self.pos_col = 1;
                    } 
                    continue;
                }
            }
            
            // Try to match tokens
            let (token_type, token_len) = if let Some(ch) = remaining.chars().next() {
                if ch.is_alphabetic() || ch == '_' {
                    self.match_id_or_kw(remaining)
                } else if ch.is_digit(10) {
                    self.match_number(remaining)
                } else {
                    self.match_syntax(remaining)
                }
            } else {
                break;
            };
            
            if let Some(tt) = token_type {
                let tok = Token {
                    line: self.pos_line,
                    col: self.pos_col,
                    len: token_len,
                    ty: tt,
                    next: None
                };
                
                // Update position
                self.cur_idx += token_len;
                self.pos_col += token_len;
                
                // Add token to the linked list
                tokens.push(tok); 
            } else {
                // Handle invalid character
                if let Some(ch) = remaining.chars().next() {
                    println!("Invalid character: {}", ch);
                    self.cur_idx += 1;
                    self.pos_col += 1;
                } else {
                    break;
                }
            } 
        }
        tokens
    } 
}

