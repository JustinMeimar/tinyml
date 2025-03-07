use crate::ast::{AstNode, AstPattern, Type, BinOp, LiteralValue};
use crate::ast_visitor::Visitable;

pub struct DebugVisitor {
    debug_depth: usize,
}

impl DebugVisitor {
    pub fn new() -> Self {
        DebugVisitor {
            debug_depth: 0
        } 
    }    
}

impl<T: Default> Visitable<T> for DebugVisitor {
     
    fn visit_literal(&mut self, lit: &LiteralValue) -> Result<T, String> {
        let (val, ty) = match lit {
            LiteralValue::Integer(s) => (s, "int"),
            LiteralValue::Boolean(s) => (s, "bool"),
            LiteralValue::String(s) => (s, "string"),
        }; 
        self.debug_depth += 1; 
        println!("{}<literal val=\"{}\" ty=\"{}\" />", 
                 "  ".repeat(self.debug_depth), val, ty);
        self.debug_depth -= 1;
        Ok(T::default())    
    }

    fn visit_program(&mut self, stmts: &Vec<Box<AstNode>>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<program>", indent);
        
        self.debug_depth += 1; 
        for stmt in stmts {
            self.visit_node(stmt)?;
        } 
        self.debug_depth -= 1;
        
        println!("{}</program>", indent);
        Ok(T::default())
    }

    fn visit_val_decl(&mut self, pat: &AstPattern, typ: &Option<Type>, exp: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<val_decl>", indent);
        
        self.debug_depth += 1; 
        self.visit_pattern(pat)?;
        self.visit_type(typ)?;
        self.visit_node(exp)?; 
        self.debug_depth -= 1;
        
        println!("{}</val_decl>", indent);
        Ok(T::default())
    }

    fn visit_fun_decl(&mut self, name: &str, clauses: &Vec<(AstPattern, Box<AstNode>)>, typ: &Option<Type>)
        -> Result<T, String>
    {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<fun_decl name=\"{}\">", indent, name);
        
        self.debug_depth += 1;
        
        self.visit_type(typ)?;
        
        for (pat, body) in clauses {
            let clause_indent = " ".repeat(2 * self.debug_depth);
            println!("{}<clause>", clause_indent);
            
            self.debug_depth += 1; 
            self.visit_pattern(pat)?;
            self.visit_node(body)?; 
            self.debug_depth -= 1;
            
            println!("{}</clause>", clause_indent);
        }
        
        self.debug_depth -= 1;
        
        println!("{}</fun_decl>", indent);
        Ok(T::default())
    }

    fn visit_if(&mut self, cond: &AstNode, then: &AstNode, else_: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<if>", indent);
        
        self.debug_depth += 1;
        
        let cond_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<condition>", cond_indent);
        self.visit_node(cond)?;
        println!("{}</condition>", cond_indent);
        
        let then_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<then>", then_indent);
        self.visit_node(then)?;
        println!("{}</then>", then_indent);
        
        let else_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<else>", else_indent);
        self.visit_node(else_)?;
        println!("{}</else>", else_indent);
        
        self.debug_depth -= 1;
        
        println!("{}</if>", indent);
        Ok(T::default())
    }

    fn visit_let(&mut self, decl: &AstNode, body: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<let>", indent);
        
        self.debug_depth += 1;
        
        let decl_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<decl>", decl_indent);
        self.visit_node(decl)?;
        println!("{}</decl>", decl_indent);
        
        let body_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<body>", body_indent);
        self.visit_node(body)?;
        println!("{}</body>", body_indent);
        
        self.debug_depth -= 1;
        
        println!("{}</let>", indent);
        Ok(T::default())
    }

    fn visit_fn(&mut self, clauses: &Vec<(AstPattern, Box<AstNode>)>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<fn>", indent);
        
        self.debug_depth += 1;
        
        for (pat, body) in clauses {
            let clause_indent = " ".repeat(2 * self.debug_depth);
            println!("{}<clause>", clause_indent);
            
            self.debug_depth += 1;
            
            self.visit_pattern(pat)?;
            self.visit_node(body)?;
            
            self.debug_depth -= 1;
            
            println!("{}</clause>", clause_indent);
        }
        
        self.debug_depth -= 1;
        
        println!("{}</fn>", indent);
        Ok(T::default())
    }

    fn visit_bin_op(&mut self, left: &AstNode, op: &BinOp, right: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<bin_op op=\"{:?}\">", indent, op);
        
        self.debug_depth += 1;
        
        let left_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<left>", left_indent);
        self.visit_node(left)?;
        println!("{}</left>", left_indent);
        
        let right_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<right>", right_indent);
        self.visit_node(right)?;
        println!("{}</right>", right_indent);
        
        self.debug_depth -= 1;
        
        println!("{}</bin_op>", indent);
        Ok(T::default())
    }

    fn visit_app(&mut self, func: &AstNode, arg: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<app>", indent);
        
        self.debug_depth += 1;
        
        let func_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<func>", func_indent);
        self.visit_node(func)?;
        println!("{}</func>", func_indent);
        
        let arg_indent = " ".repeat(2 * self.debug_depth);
        println!("{}<arg>", arg_indent);
        self.visit_node(arg)?;
        println!("{}</arg>", arg_indent);
        
        self.debug_depth -= 1;
        
        println!("{}</app>", indent);
        Ok(T::default())
    }

    fn visit_id(&mut self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<id value=\"{}\">", indent, name);
        println!("{}</id>", indent);
        Ok(T::default())
    }

    fn visit_var(&mut self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<var value=\"{}\">", indent, name);
        println!("{}</var>", indent);
        Ok(T::default())
    }

    fn visit_tuple(&mut self, elements: &Vec<Box<AstNode>>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<tuple size=\"{}\">", indent, elements.len());
        
        self.debug_depth += 1; 
        for elem in elements {
            self.visit_node(elem)?;
        } 
        self.debug_depth -= 1;
        
        println!("{}</tuple>", indent);
        Ok(T::default())
    }

    fn visit_list(&mut self, elements: &Vec<Box<AstNode>>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<list size=\"{}\">", indent, elements.len());
        
        self.debug_depth += 1; 
        for elem in elements {
            self.visit_node(elem)?;
        } 
        self.debug_depth -= 1; 
        println!("{}</list>", indent);
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
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<literal_pattern>", indent);
        println!("{}</literal_pattern>", indent);
        Ok(T::default())
    }

    fn visit_id_pattern(&mut self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<id_pattern value=\"{}\">", indent, name);
        println!("{}</id_pattern>", indent);
        Ok(T::default())
    }

    fn visit_wildcard_pattern(&mut self) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<wildcard_pattern>", indent);
        println!("{}</wildcard_pattern>", indent);
        Ok(T::default())
    }

    fn visit_var_pattern(&mut self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<var_pattern value=\"{}\">", indent, name);
        println!("{}</var_pattern>", indent);
        Ok(T::default())
    }

    fn visit_pair_pattern(&mut self, first: &AstPattern, second: &AstPattern) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<pair_pattern>", indent);
        
        self.debug_depth += 1; 
        self.visit_pattern(first)?;
        self.visit_pattern(second)?; 
        self.debug_depth -= 1;
        
        println!("{}</pair_pattern>", indent);
        Ok(T::default())
    }

    fn visit_type(&mut self, typ: &Option<Type>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        
        if let Some(t) = typ {
            println!("{}<type value=\"{:?}\">", indent, t);
            println!("{}</type>", indent);
        } else {
            println!("{}<type value=\"none\">", indent);
            println!("{}</type>", indent);
        }   
        Ok(T::default())
    } 
}

