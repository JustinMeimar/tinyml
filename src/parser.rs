use crate::{ast::{AstNode, AstPattern, Type, BinOp}, lexer::{Token, TokenType}};
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
    pub fn parse(&mut self) -> Result<Box<AstNode>, String> {
        self.tokens.iter().for_each(|x| println!(" == {:?}", x.ty));
        
        let mut decls = Vec::new();
        while self.peek().is_some() {
            for decl in self.parse_decls()? {
                decls.push(Box::new(decl));
            }
        }
        Ok(Box::new(AstNode::Program(decls)))
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let prefix = match self.peek() {
            Some(TokenType::TypeInt) => {
                self.consume();
                Ok(Type::Int)
            },
            Some(TokenType::TypeBool) => {
                self.consume();
                Ok(Type::Bool)
            },
            Some(TokenType::TypeChar) => {
                self.consume();
                Ok(Type::Char)
            },
            Some(TokenType::TypeString) => {
                self.consume();
                Ok(Type::String)
            },
            Some(TokenType::SingleQuote) => {
                self.consume(); // Consume the single quote
                match self.peek() {
                    Some(TokenType::Id(id)) => {
                        self.consume();
                        Ok(Type::Var(id))
                    },
                    _ => Err("Failed to parse rule: var: ' ID".to_string()),
                }
            },
            Some(TokenType::LeftParen) => {
                self.consume(); // Consume '('
                let type1 = self.parse_type()?;
                self.expect(TokenType::Comma)?;
                let type2 = self.parse_type()?;
                self.expect(TokenType::RightParen)?;
                Ok(Type::Product(Box::new(type1), Box::new(type2)))
            },
            _ => Err("Expected a type".to_string()),
        }?;

        match self.peek() {
            Some(TokenType::Arrow) => {
                self.consume(); // Consume '->'
                let return_type = self.parse_type()?;
                Ok(Type::Arrow(Box::new(prefix), Box::new(return_type)))
            },
            Some(TokenType::Multiply) => {
                self.consume(); // Consume '*'
                let second_type = self.parse_type()?;
                Ok(Type::Product(Box::new(prefix), Box::new(second_type)))
            },
            _ => Ok(prefix), // No suffix, just return the prefix type
        }
    }

    fn parse_atom(&mut self) -> Result<AstNode, String> {
        match self.peek() {
            Some(TokenType::Integer(n)) => {
                self.consume();
                // Note: AstPattern doesn't have an IntLit variant, so we use Id
                Ok(AstNode::Id(n.to_string()))
            },
            Some(TokenType::Bool(b)) => {
                self.consume();
                // Note: AstPattern doesn't have a BoolLit variant, so we use Id
                Ok(AstNode::Id(b.to_string()))
            },
            Some(TokenType::String(s)) => {
                self.consume();
                // Note: AstPattern doesn't have a StringLit variant, so we use Id
                Ok(AstNode::Id(s))
            },
            Some(TokenType::Id(id)) => {
                self.consume();
                Ok(AstNode::Id(id))
            },
            Some(TokenType::LeftParen) => {
                self.consume(); // Consume '('
                
                // Check for empty tuple '()'
                if let Some(TokenType::RightParen) = self.peek() {
                    self.consume(); // Consume ')'
                    // Using a 0-element Tuple for Unit
                    return Ok(AstNode::Tuple(Vec::new()));
                }
                
                // Parse an expression
                let expr = self.parse_expr()?;
                
                // Check if it's a tuple '(exp, exp, ...)'
                if let Some(TokenType::Comma) = self.peek() {
                    self.consume(); // Consume ','
                    let mut expressions = vec![Box::new(expr)];
                    
                    // Parse second expression (required for tuple)
                    expressions.push(Box::new(self.parse_expr()?));
                    
                    // Parse any additional expressions
                    while let Some(TokenType::Comma) = self.peek() {
                        self.consume(); // Consume ','
                        expressions.push(Box::new(self.parse_expr()?));
                    }
                    
                    self.expect(TokenType::RightParen)?;
                    Ok(AstNode::Tuple(expressions))
                } else {
                    self.expect(TokenType::RightParen)?;
                    Ok(expr)
                }
            },
            Some(TokenType::LeftParen) => {
                self.consume(); // Consume '['
                
                // Check for empty list '[]'
                if let Some(TokenType::RightParen) = self.peek() {
                    self.consume(); // Consume ']'
                    return Ok(AstNode::List(Vec::new()));
                }
                
                // Parse a non-empty list
                let mut items = Vec::new();
                items.push(Box::new(self.parse_expr()?));
                
                // Parse additional items
                while let Some(TokenType::Comma) = self.peek() {
                    self.consume(); // Consume ','
                    items.push(Box::new(self.parse_expr()?));
                }
                
                self.expect(TokenType::RightBracket)?;
                Ok(AstNode::List(items))
            },
            _ => Err("Expected an atom".to_string()),
        }
    }

    fn parse_app_expr(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_atom()?;
        
        // Keep applying atoms as long as we see them
        // We need to look ahead to see if the next token could be the start of an atom
        while self.could_start_atom() {
            let atom = self.parse_atom()?;
            expr = AstNode::App {
                func: Box::new(expr),
                arg: Box::new(atom),
            };
        }
        
        Ok(expr)
    }

    // Helper method to check if the next token could start an atom
    fn could_start_atom(&self) -> bool {
        match self.peek() {
            Some(TokenType::Integer(_)) |
            Some(TokenType::Bool(_)) |
            Some(TokenType::String(_)) |
            Some(TokenType::Id(_)) |
            Some(TokenType::LeftParen) |
            Some(TokenType::LeftBracket) => true,
            _ => false,
        }
    }

    fn parse_mul_expr(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_app_expr()?;
        
        loop {
            match self.peek() {
                Some(TokenType::Multiply) => {
                    self.consume();
                    let right = self.parse_app_expr()?;
                    left = AstNode::BinOp {
                        left: Box::new(left),
                        op: BinOp::Mul,
                        right: Box::new(right),
                    };
                },
                Some(TokenType::Divide) => {
                    self.consume();
                    let right = self.parse_app_expr()?;
                    left = AstNode::BinOp {
                        left: Box::new(left),
                        op: BinOp::Div,
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(left)
    }

    fn parse_add_expr(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_mul_expr()?;
        
        loop {
            match self.peek() {
                Some(TokenType::Plus) => {
                    self.consume();
                    let right = self.parse_mul_expr()?;
                    left = AstNode::BinOp {
                        left: Box::new(left),
                        op: BinOp::Add,
                        right: Box::new(right),
                    };
                },
                Some(TokenType::Minus) => {
                    self.consume();
                    let right = self.parse_mul_expr()?;
                    left = AstNode::BinOp {
                        left: Box::new(left),
                        op: BinOp::Sub,
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_comp_expr(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_add_expr()?;
        
        loop {
            let op = match self.peek() {
                Some(TokenType::Less)           => Some(BinOp::Lt),
                Some(TokenType::StrictLess)     => Some(BinOp::Lte),
                Some(TokenType::Greater)        => Some(BinOp::Gt),
                Some(TokenType::StrictGreater)  => Some(BinOp::Gte),
                Some(TokenType::CompEqual)      => Some(BinOp::Eq),
                Some(TokenType::CompNotEqual)   => Some(BinOp::Eq),
                _ => None,
            };
            
            if let Some(bin_op) = op {
                self.consume();
                let right = self.parse_add_expr()?;
                left = AstNode::BinOp {
                    left: Box::new(left),
                    op: bin_op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    

    fn parse_match(&mut self) -> Result<Vec<(AstPattern, Box<AstNode>)>, String> {
        let mut arms = Vec::new();
        
        let pattern = self.parse_pattern()?;
        self.expect(TokenType::FatArrow)?; // =>
        let expr = Box::new(self.parse_expr()?);
        arms.push((pattern, expr));
        
        // Parse additional arms if present
        while let Some(TokenType::Bar) = self.peek() {
            self.consume();
            let pattern = self.parse_pattern()?;
            self.expect(TokenType::FatArrow)?;
            let expr = Box::new(self.parse_expr()?);
            arms.push((pattern, expr));
        }
        
        Ok(arms)
    }

    // Complete the expr parsing
    fn parse_expr(&mut self) -> Result<AstNode, String> {
        match self.peek() {
            Some(TokenType::If) => {
                self.consume(); // Eat 'if'
                let cond = Box::new(self.parse_expr()?);
                self.expect(TokenType::Then)?;
                let then_expr = Box::new(self.parse_expr()?);
                self.expect(TokenType::Else)?;
                let else_expr = Box::new(self.parse_expr()?);
                Ok(AstNode::If {
                    cond,
                    then: then_expr,
                    else_: else_expr,
                })
            },
            Some(TokenType::Let) => {
                self.consume(); // Eat 'let'
                let decl = Box::new(self.parse_decl()?);
                self.expect(TokenType::In)?;
                let body = Box::new(self.parse_expr()?);
                self.expect(TokenType::End)?;
                Ok(AstNode::Let {
                    decl,
                    body,
                })
            },
            Some(TokenType::Fun) => {
                self.consume(); // Eat 'fn'
                let clauses = self.parse_match()?;
                Ok(AstNode::Fn {
                    clauses,
                })
            },
            _ => self.parse_comp_expr(),
        }
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
                let exp = Box::new(self.parse_expr()?);
                Ok(AstNode::ValDecl {
                    pat,
                    typ,
                    exp,
                })
            },
            Some(TokenType::Fun) => {
                self.consume(); // Eat 'fun'
                if let Some(TokenType::Id(name)) = self.peek() {
                    let id = name.clone();
                    self.consume(); // Consume the ID
                    let clauses = self.parse_match()?;
                    let typ = if self.peek() == Some(TokenType::Colon) {
                        self.consume();
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    Ok(AstNode::FunDecl {
                        name: id,
                        clauses,
                        typ,
                    })
                } else {
                    Err("Expected identifier after 'fun'".to_string())
                }
            },
            _ => Err("Failed to parse declaration".to_string())
        }
    }

    fn parse_decls(&mut self) -> Result<Vec<AstNode>, String> {
        // Multiple SC separated declarations may be made on the same line
        let mut decls = Vec::new();
        loop {
            let dec =self.parse_decl()?;
            decls.push(dec); 
            
            // Handle extra declarations
            match self.peek() {
                Some(TokenType::SemiColon) => {
                    self.consume();
                    continue
                },
                _ => break 
            } 
        }
        Ok(decls) 
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
            },
            Some(TokenType::Integer(_)) | 
            Some(TokenType::Bool(_)) | 
            Some(TokenType::String(_)) => {
                self.consume();
                Ok(AstPattern::Literal)
            },
            Some(TokenType::SingleQuote) => {
                self.consume();
                if let Some(TokenType::Id(id)) = self.peek() {
                    self.consume();
                    Ok(AstPattern::Var(id))
                } else {
                    Err("Expected identifier after single quote in pattern".to_string())
                }
            },
            _ => Err("Invalid pattern".to_string()),
        }
    }
}

