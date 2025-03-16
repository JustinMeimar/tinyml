use crate::ast_visitor::Visitable;

pub struct DefVisitor {}

impl DefVisitor {
    pub fn new() -> Self {
        DefVisitor {}
    }
}

impl<T: Default> Visitable<T> for DefVisitor {
    
    fn visit_val_decl(&mut self, pat: &crate::ast::AstPattern, typ: &Option<crate::ast::Type>, exp: &crate::ast::AstNode)
            -> Result<T, String> {

        println!("Visit val decl!");

        Ok(T::default())
    }
}  

