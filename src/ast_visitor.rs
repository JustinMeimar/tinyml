use crate::ast::{AstNode, AstPattern, BinOp, LiteralValue, Type};

pub trait Visitable<T: Default> {
    
    fn visit(&mut self, ast: &AstNode) -> Result<T, String> {
        self.visit_node(ast);
        Ok(T::default())
    } 
    
    fn visit_node(&mut self, node: &AstNode) -> Result<T, String> {
        match node {
            AstNode::Program(stmts) => self.visit_program(stmts),
            AstNode::ValDecl { pat, typ, exp } => self.visit_val_decl(pat, typ, exp),
            AstNode::FunDecl { name, clauses, typ } => self.visit_fun_decl(name, clauses, typ),
            AstNode::If { cond, then, else_ } => self.visit_if(cond, then, else_),
            AstNode::Let { decl, body } => self.visit_let(decl, body),
            AstNode::Fn { clauses } => self.visit_fn(clauses),
            AstNode::BinOp { left, op, right } => self.visit_bin_op(left, op, right),
            AstNode::App { func, arg } => self.visit_app(func, arg),
            AstNode::Id(name) => self.visit_id(name),
            AstNode::Var(name) => self.visit_var(name),
            AstNode::Tuple(elements) => self.visit_tuple(elements),
            AstNode::List(elements) => self.visit_list(elements),
            AstNode::Literal(lit) => self.visit_literal(lit),
        }
    }
    
    fn visit_program(&mut self, stmts: &Vec<Box<AstNode>>) -> Result<T, String> {
        for stmt in stmts {
            self.visit_node(stmt)?;
        }
        Ok(T::default())
    }
    
    fn visit_fun_decl(
        &mut self, _name: &str,
        clauses: &Vec<(AstPattern, Box<AstNode>)>,
        typ: &Option<Type>) -> Result<T, String>
    {    
        for (pattern, expr) in clauses {
            self.visit_pattern(pattern)?;
            self.visit_node(expr)?;
        }
        
        if let Some(t) = typ {
            self.visit_type(&Some(t.clone()))?;
        }
        
        Ok(T::default())
    }
     
    fn visit_val_decl(&mut self, pat: &AstPattern, typ: &Option<Type>, exp: &AstNode)
        -> Result<T, String>
    {    
        self.visit_pattern(pat)?; 
        if let Some(t) = typ {
            self.visit_type(&Some(t.clone()))?;
        } 
        self.visit_node(exp)?;
        
        Ok(T::default())
    }
    
    fn visit_if(&mut self, cond: &AstNode, then: &AstNode, else_: &AstNode)
        -> Result<T, String>
    {    
        self.visit_node(cond)?;
        self.visit_node(then)?;
        self.visit_node(else_)?;
        
        Ok(T::default())
    }
    
    fn visit_let(&mut self, decl: &AstNode, body: &AstNode) -> Result<T, String> {
        self.visit_node(decl)?;
        self.visit_node(body)?;
        Ok(T::default())
    }
    
    fn visit_fn(&mut self, clauses: &Vec<(AstPattern, Box<AstNode>)>)
        -> Result<T, String>
    {
        for (pattern, expr) in clauses {
            self.visit_pattern(pattern)?;
            self.visit_node(expr)?;
        }
        Ok(T::default())
    }
    
    fn visit_bin_op(&mut self, left: &AstNode, op: &BinOp, right: &AstNode)
        -> Result<T, String> {
        self.visit_node(left)?;
        self.visit_node(right)?;
        Ok(T::default())
    }
    
    fn visit_app(&mut self, func: &AstNode, arg: &AstNode)
        -> Result<T, String> {
        self.visit_node(func)?;
        self.visit_node(arg)?;
        Ok(T::default())
    }
    
    fn visit_id(&mut self, _name: &str) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_var(&mut self, _name: &str) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_tuple(&mut self, elements: &Vec<Box<AstNode>>) -> Result<T, String> {
        for element in elements {
            self.visit_node(element)?;
        }
        Ok(T::default())
    }
    
    fn visit_list(&mut self, elements: &Vec<Box<AstNode>>) -> Result<T, String> {
        for element in elements {
            self.visit_node(element)?;
        }
        Ok(T::default())
    }
    
    fn visit_pattern(&mut self, pat: &AstPattern) -> Result<T, String> {
        match pat {
            AstPattern::Literal => self.visit_literal_pattern(),
            AstPattern::Id(name) => self.visit_id_pattern(name),
            AstPattern::Wildcard => self.visit_wildcard_pattern(),
            AstPattern::Var(name) => self.visit_var_pattern(name),
            AstPattern::Pair(first, second) => self.visit_pair_pattern(first, second),
        }
    }
    
    fn visit_literal_pattern(&mut self) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_id_pattern(&mut self, _name: &str) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_wildcard_pattern(&mut self) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_var_pattern(&mut self, _name: &str) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_pair_pattern(&mut self, first: &AstPattern, second: &AstPattern) -> Result<T, String> {
        self.visit_pattern(first)?;
        self.visit_pattern(second)?;
        Ok(T::default())
    }
    
    fn visit_type(&mut self, _typ: &Option<Type>) -> Result<T, String> {
        Ok(T::default())
    }
    
    fn visit_literal(&mut self, _lit_node: &LiteralValue) -> Result<T, String> {
        Ok(T::default())
    }
}

