use std::fmt;

#[macro_export]
macro_rules! parse_error {
    ($kind:expr, $msg:expr) => {
        $crate::ParseError::new($kind, $msg.to_string(), None)
    };
    ($kind:expr, $msg:expr, $pos:expr) => {
        $crate::ParseError::new($kind, $msg.to_string(), Some($pos))
    };
}

#[derive(Debug)]
pub enum ErrKind {
    UnexpectedToken,
    InvalidPattern,
    InvalidExpression,
    InvalidDeclaration,
    UnexpectedEOF,
}

impl fmt::Display for ErrKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrKind::InvalidPattern => write!(f, "Invalid pattern"),
            ErrKind::InvalidExpression => write!(f, "Invalid expression"),
            ErrKind::InvalidDeclaration => write!(f, "Invalid declaration"),
            ErrKind::UnexpectedEOF => write!(f, "Unexpected end of file"),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ErrKind,
    pub msg: String,
    pub pos: Option<usize>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos_display = match self.pos {
            Some(pos) => pos.to_string(),
            None => "unknown".to_string(),
        }; 
        write!(f, "ParseError: {} - {} (position: {})", self.kind, self.msg, pos_display)
    }
}

impl std::error::Error for ParseError {}

// Direct constructors are simpler than using a separate params struct
impl ParseError {
    pub fn new(kind: ErrKind, msg: String, pos: Option<usize>) -> Self {
        Self { kind, msg, pos }
    }
    
    pub fn with_pos(kind: ErrKind, msg: String, pos: usize) -> Self {
        Self { kind, msg, pos: Some(pos) }
    }
}

