use std::collections::HashMap;
use jolang_shared::ir::instructions::operand::{BlkId, Imm64};

#[derive(PartialEq, Debug)]
pub enum ScopeKind {
    // the first Scope contains globals
    Root,
    Block,
    Loop
}

#[derive(Debug)]
pub struct Scope {
    // name : (offset, size)
    variables : HashMap<String, (u64, u64)>,
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

    pub fn decl_var(&mut self, name : String, offset : u64, size : u64) {
        self.variables.insert(name, (offset, size));
    }

    pub fn get_var_offset(&self, name : &String) -> Option<u64> {
        self.variables.get(name).copied().map(|x| x.0)
    }

    pub fn get_var_size(&self, name : &String) -> Option<u64> {
        self.variables.get(name).copied().map(|x| x.1)
    }

    pub fn update_var(&mut self, name : &String, offset : u64) {
        self.variables.get_mut(name).unwrap().0 = offset;
    }

    pub fn var_sizes(&self) -> Vec<u64> {
        self.get_vars().values()
            .map(|v| v.1)
            .collect::<Vec<_>>()
    }

    pub fn get_vars(&self) -> &HashMap<String, (u64, u64)> {
        &self.variables
    }

    pub fn get_vars_mut(&mut self) -> &mut HashMap<String, (u64, u64)> {
        &mut self.variables
    }
}
