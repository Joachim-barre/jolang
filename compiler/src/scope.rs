use std::collections::HashMap;
use jolang_shared::ir::instructions::operand::{BlkId, Imm};

#[derive(PartialEq, Debug)]
pub enum ScopeKind {
    // the first Scope contains globals
    Root,
    Block,
    Loop
}

#[derive(Debug)]
pub struct Scope {
    // the Imm is the offset from the start of the stack not the top
    variables : HashMap<String, u64>,
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

    pub fn decl_var(&mut self, name : String, offset : u64) {
        self.variables.insert(name, offset);
    }

    pub fn get_var(&self, name : &String) -> Option<u64> {
        self.variables.get(name).copied()
    }
}
