use std::fmt;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken,
    InvalidPattern,
    InvalidExpression,
    InvalidDeclaration,
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub position: Option<usize>,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, message: String) -> Self {
        ParseError {
            kind,
            message,
            position: None,
        }
    }
    
    pub fn with_position(mut self, position: usize) -> Self {
        self.position = Some(position);
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)?;
        
        if let Some(pos) = self.position {
            write!(f, " at position {}", pos)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for ParseError {}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        ParseError::new(ParseErrorKind::UnexpectedToken, message)
    }
}
