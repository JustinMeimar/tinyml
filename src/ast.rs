#[derive(Debug)]
pub enum AstNode {
    Program(Vec<Box<AstNode>>),
    ValDecl {
        pat: AstPattern,
        typ: Option<Type>,
        exp: Box<AstNode>,
    },
    FunDecl {
        name: String,
        clauses: Vec<(AstPattern, Box<AstNode>)>,
        typ: Option<Type>,
    },
    If {
        cond: Box<AstNode>,
        then: Box<AstNode>,
        else_: Box<AstNode>,
    },
    Let {
        decl: Box<AstNode>,
        body: Box<AstNode>,
    },
    Fn {
        clauses: Vec<(AstPattern, Box<AstNode>)>,
    },
    BinOp {
        left: Box<AstNode>,
        op: String,
        right: Box<AstNode>,
    },
    App {
        func: Box<AstNode>,
        arg: Box<AstNode>,
    },
    Id(String),
    Var(String),
    Tuple(Vec<Box<AstNode>>),
    List(Vec<Box<AstNode>>),
}

#[derive(Debug)]
pub enum AstPattern {
    Literal,
    Id(String),
    Wildcard,
    Var(String),
    Pair(Box<AstPattern>, Box<AstPattern>),
}

#[derive(Debug)]
pub enum Type {
    Int(String),
    Char(String),
    String(String),
    Var(String),
    Arrow(Box<Type>, Box<Type>),
    Product(Box<Type>, Box<Type>),
}

