
impl<T> DebugVisitor<T> {
    pub fn new(root: Box<AstNode>) -> Self {
        DebugVisitor {
            root,
            debug_depth: 0
        } 
    }
    
    pub fn visit(&self) -> Result<T, String> where Self: Visitable<T> {
        self.visit_node(&self.root)
    }
    
    fn visit_node(&self, node: &AstNode) -> Result<T, String> 
        where Self: Visitable<T>
    {
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
        }
    }
}



impl<T: Default> Visitable<T> for DebugVisitor<T> {
    fn visit_program(&self, stmts: &Vec<Box<AstNode>>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?program>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        for stmt in stmts {
            visitor.visit_node(stmt)?;
        }
        
        println!("{}</?program>", indent);
        Ok(T::default())
    }

    fn visit_val_decl(&self, pat: &AstPattern, typ: &Option<Type>, exp: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?val_decl>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        visitor.visit_pattern(pat)?;
        visitor.visit_type(typ)?;
        visitor.visit_node(exp)?;
        
        println!("{}</?val_decl>", indent);
        Ok(T::default())
    }

    fn visit_fun_decl(&self, name: &str, clauses: &Vec<(AstPattern, Box<AstNode>)>, typ: &Option<Type>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?fun_decl name=\"{}\">", indent, name);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        visitor.visit_type(typ)?;
        
        for (pat, body) in clauses {
            let clause_indent = " ".repeat(2 * visitor.debug_depth);
            println!("{}<?clause>", clause_indent);
            
            let mut clause_visitor = DebugVisitor {
                root: self.root.clone(),
                debug_depth: visitor.debug_depth + 1,
            };
            
            clause_visitor.visit_pattern(pat)?;
            clause_visitor.visit_node(body)?;
            
            println!("{}</?clause>", clause_indent);
        }
        
        println!("{}</?fun_decl>", indent);
        Ok(T::default())
    }

    fn visit_if(&self, cond: &AstNode, then: &AstNode, else_: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?if>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        let cond_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?condition>", cond_indent);
        visitor.visit_node(cond)?;
        println!("{}</?condition>", cond_indent);
        
        let then_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?then>", then_indent);
        visitor.visit_node(then)?;
        println!("{}</?then>", then_indent);
        
        let else_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?else>", else_indent);
        visitor.visit_node(else_)?;
        println!("{}</?else>", else_indent);
        
        println!("{}</?if>", indent);
        Ok(T::default())
    }

    fn visit_let(&self, decl: &AstNode, body: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?let>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        let decl_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?decl>", decl_indent);
        visitor.visit_node(decl)?;
        println!("{}</?decl>", decl_indent);
        
        let body_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?body>", body_indent);
        visitor.visit_node(body)?;
        println!("{}</?body>", body_indent);
        
        println!("{}</?let>", indent);
        Ok(T::default())
    }

    fn visit_fn(&self, clauses: &Vec<(AstPattern, Box<AstNode>)>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?fn>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        for (pat, body) in clauses {
            let clause_indent = " ".repeat(2 * visitor.debug_depth);
            println!("{}<?clause>", clause_indent);
            
            let mut clause_visitor = DebugVisitor {
                root: self.root.clone(),
                debug_depth: visitor.debug_depth + 1,
            };
            
            clause_visitor.visit_pattern(pat)?;
            clause_visitor.visit_node(body)?;
            
            println!("{}</?clause>", clause_indent);
        }
        
        println!("{}</?fn>", indent);
        Ok(T::default())
    }

    fn visit_bin_op(&self, left: &AstNode, op: &BinOp, right: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?bin_op op=\"{:?}\">", indent, op);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        let left_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?left>", left_indent);
        visitor.visit_node(left)?;
        println!("{}</?left>", left_indent);
        
        let right_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?right>", right_indent);
        visitor.visit_node(right)?;
        println!("{}</?right>", right_indent);
        
        println!("{}</?bin_op>", indent);
        Ok(T::default())
    }

    fn visit_app(&self, func: &AstNode, arg: &AstNode) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?app>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        let func_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?func>", func_indent);
        visitor.visit_node(func)?;
        println!("{}</?func>", func_indent);
        
        let arg_indent = " ".repeat(2 * visitor.debug_depth);
        println!("{}<?arg>", arg_indent);
        visitor.visit_node(arg)?;
        println!("{}</?arg>", arg_indent);
        
        println!("{}</?app>", indent);
        Ok(T::default())
    }

    fn visit_id(&self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?id value=\"{}\">", indent, name);
        println!("{}</?id>", indent);
        Ok(T::default())
    }

    fn visit_var(&self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?var value=\"{}\">", indent, name);
        println!("{}</?var>", indent);
        Ok(T::default())
    }

    fn visit_tuple(&self, elements: &Vec<Box<AstNode>>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?tuple size=\"{}\">", indent, elements.len());
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        for elem in elements {
            visitor.visit_node(elem)?;
        }
        
        println!("{}</?tuple>", indent);
        Ok(T::default())
    }

    fn visit_list(&self, elements: &Vec<Box<AstNode>>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?list size=\"{}\">", indent, elements.len());
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        for elem in elements {
            visitor.visit_node(elem)?;
        }
        
        println!("{}</?list>", indent);
        Ok(T::default())
    }

    fn visit_pattern(&self, pat: &AstPattern) -> Result<T, String> {
        match pat {
            AstPattern::Literal => self.visit_literal_pattern(),
            AstPattern::Id(name) => self.visit_id_pattern(name),
            AstPattern::Wildcard => self.visit_wildcard_pattern(),
            AstPattern::Var(name) => self.visit_var_pattern(name),
            AstPattern::Pair(first, second) => self.visit_pair_pattern(first, second),
        }
    }

    fn visit_literal_pattern(&self) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?literal_pattern>", indent);
        println!("{}</?literal_pattern>", indent);
        Ok(T::default())
    }

    fn visit_id_pattern(&self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?id_pattern value=\"{}\">", indent, name);
        println!("{}</?id_pattern>", indent);
        Ok(T::default())
    }

    fn visit_wildcard_pattern(&self) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?wildcard_pattern>", indent);
        println!("{}</?wildcard_pattern>", indent);
        Ok(T::default())
    }

    fn visit_var_pattern(&self, name: &str) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?var_pattern value=\"{}\">", indent, name);
        println!("{}</?var_pattern>", indent);
        Ok(T::default())
    }

    fn visit_pair_pattern(&self, first: &AstPattern, second: &AstPattern) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        println!("{}<?pair_pattern>", indent);
        
        let mut visitor = DebugVisitor {
            root: self.root.clone(),
            debug_depth: self.debug_depth + 1,
        };
        
        visitor.visit_pattern(first)?;
        visitor.visit_pattern(second)?;
        
        println!("{}</?pair_pattern>", indent);
        Ok(T::default())
    }

    fn visit_type(&self, typ: &Option<Type>) -> Result<T, String> {
        let indent = " ".repeat(2 * self.debug_depth);
        
        if let Some(t) = typ {
            println!("{}<?type value=\"{:?}\">", indent, t);
            println!("{}</?type>", indent);
        } else {
            println!("{}<?type value=\"none\">", indent);
            println!("{}</?type>", indent);
        }
        
        Ok(T::default())
    }
}
