use crate::{ast::{AstNode, AstPattern, Type}, lexer::{Token, TokenType}};
use std::result::Result;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { 
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<TokenType> {
        self.tokens.get(self.pos).map(|t| t.ty.clone())
    }
    
    fn consume(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }
    
    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        if let Some(token) = self.consume() {
            if token.ty == expected {
                Ok(())
            } else {
                Err(format!("Expected '{:?}', got '{:?}'", expected, token))
            }
        } else {
            Err(format!("Expected '{:?}', got EOF", expected))
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        self.tokens.iter().for_each(|x| println!("{:?}", x.ty));
        let mut decls = Vec::new();
        while self.peek().is_some() {
            decls.push(Box::new(self.parse_decl()?));
        }
        Ok(AstNode::Program(decls))
    }
   
    fn parse_expr(&mut self) -> Result<AstNode, String> {
        Err(String::from("Todo: Expression"))
    }
    
    fn parse_pattern(&mut self) -> Result<AstPattern, String> {
        match self.peek() {
            Some(TokenType::Wildcard) => {
                self.consume();
                Ok(AstPattern::Wildcard)
            },
            Some(TokenType::LeftParen) => {
                self.consume();
                let p1 = self.parse_pattern()?;
                self.expect(TokenType::Comma)?;
                let p2 = self.parse_pattern()?;
                self.expect(TokenType::RightParen)?;
                Ok(AstPattern::Pair(Box::new(p1), Box::new(p2)))
            },
            Some(TokenType::Id(id)) => {
                self.consume();
                Ok(AstPattern::Id(id))
            }
            _ => Err("Parser failed!".to_string()),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        Err(String::from("Todo: Types!"))
    }

    fn parse_decl(&mut self) -> Result<AstNode, String> {
        match self.peek() {
            Some(TokenType::Val) => {
                self.consume(); // Eat 'val'
                let pat = self.parse_pattern()?;
                let typ = if self.peek() == Some(TokenType::Colon) {
                    self.consume();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.expect(TokenType::Equal)?;
                let exp = self.parse_expr()?;
                Ok(AstNode::ValDecl {
                    pat,
                    typ,
                    exp: Box::new(exp),
                })
            },
            Some(TokenType::Let) => {
                Err(String::from("Todo: Let"))
            },
            _ => {
                Err(String::from("Todo: Other Decl"))
            }
        }
    }
}

