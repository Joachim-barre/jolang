use std::collections::HashMap;
use jolang_shared::ir::instructions::operand::VarId;

pub enum ScopeKind {
    // the first Scope contains globals
    Root,
    Block,
    Loop
}

pub struct Scope {
    variables : HashMap<String, VarId>,
    kind : ScopeKind
}

impl Scope {
    pub fn new(kind : ScopeKind) -> Self {
        Self { 
            variables: HashMap::new(), 
            kind
        }
    }
}
