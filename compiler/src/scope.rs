use std::collections::HashMap;
use jolang_shared::ir::instructions::operand::{BlkId, VarId};

#[derive(PartialEq, Debug)]
pub enum ScopeKind {
    // the first Scope contains globals
    Root,
    Block,
    Loop
}

#[derive(Debug)]
pub struct Scope {
    variables : HashMap<String, VarId>,
    pub kind : ScopeKind,
    pub block : BlkId,
    pub exit : BlkId
}

impl Scope {
    pub fn new(kind : ScopeKind, block : BlkId, exit : BlkId) -> Self {
        Self { 
            variables: HashMap::new(), 
            kind,
            block,
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
