use std::collections::HashMap;
use jolang_shared::ir::instructions::operand::{BlkId, VarId};

pub enum ScopeKind {
    // the first Scope contains globals
    Root,
    Block,
    Loop
}

pub struct Scope {
    variables : HashMap<String, VarId>,
    kind : ScopeKind,
    exit : Option<BlkId>
}

impl Scope {
    pub fn new(kind : ScopeKind, exit : Option<BlkId>) -> Self {
        Self { 
            variables: HashMap::new(), 
            kind,
            exit
        }
    }

    pub fn decl_var(&mut self, name : String, id : VarId) {
        self.variables.insert(name, id);
    }

    pub fn get_var(&self, name : &String) -> Option<VarId> {
        self.variables.get(name).copied()
    }
}
