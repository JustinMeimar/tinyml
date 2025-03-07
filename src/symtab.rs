use crate::ast::Type;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Scope {
    symbols: Vec<Symbol>,
    child_scopes: Vec<Rc<RefCell<Scope>>>,
    parent_scope: Option<Weak<RefCell<Scope>>>
}

pub struct SymbolTable {
    global_scope: Rc<RefCell<Scope>>,
    
    /// stack to keep track of current traversal.
    scope_stack: Vec<Rc<RefCell<Scope>>>,
}

#[derive(Clone)]
pub struct Symbol {
    id: String,
    ty: Type,
    scope_ptr: Weak<RefCell<Scope>> // pointer to scope this symbol is defined in 
}

impl Scope {
    fn new(parent_scope: Option<Weak<RefCell<Scope>>>) -> Self {
        Scope {
            symbols: Vec::new(),
            child_scopes: Vec::new(),
            parent_scope,
        }
    }

    fn add_child_scope(&mut self, scope: Rc<RefCell<Scope>>) {
        self.child_scopes.push(scope); 
    }

    fn resolve(&self, id: &str) -> Option<Symbol> {
        self.symbols
            .iter()
            .find(|s| s.id == id)
            .cloned()
    }
}

impl SymbolTable {
    
    fn new() -> Self {
        let global = Rc::new(RefCell::new(Scope::new(None)));
        SymbolTable {
            global_scope: global.clone(), // no parent scope for global
            scope_stack: vec![global],
        }
    }

    fn resolve(&self, id: &str) -> Option<Symbol> {
        self.scope_stack
            .iter()
            .rev()
            .find_map(|scope| scope.borrow().resolve(id))
    }

    fn push_scope(&mut self, scope: Rc<RefCell<Scope>>) {
        self.scope_stack.push(scope)
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }
}

