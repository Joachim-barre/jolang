use std::collections::HashMap;
use jolang_shared::ir::instructions::operand::{BlkId, Imm64};

#[derive(PartialEq, Debug)]
pub enum ScopeKind {
    // the first Scope contains globals
    Root,
    Block,
    Loop
}

#[derive(Debug, Clone, Copy)]
pub enum VarType {
    SignedInt(u64),
    UnsignedInt(u64)
}

#[derive(Debug, Clone, Copy)]
pub struct Variable {
    pub _type : VarType,
    pub pos : u64
}

#[derive(Debug)]
pub struct Scope {
    variables : HashMap<String, Variable>,
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

    pub fn decl_var(&mut self, name : String, offset : u64, _type : VarType) {
        self.variables.insert(name, Variable { pos : offset, _type });
    }

    pub fn get_var_offset(&self, name : &String) -> Option<u64> {
        self.variables.get(name).copied().map(|v| v.pos)
    }

    pub fn get_var_type(&self, name : &String) -> Option<VarType> {
        self.variables.get(name).copied().map(|v| v._type)
    }

    pub fn update_var(&mut self, name : &String, offset : u64) {
        self.variables.get_mut(name).unwrap().pos = offset;
    }

    pub fn var_types(&self) -> Vec<VarType> {
        self.get_vars().values()
            .map(|v| v._type)
            .collect::<Vec<_>>()
    }

    pub fn get_vars(&self) -> &HashMap<String, Variable> {
        &self.variables
    }

    pub fn get_vars_mut(&mut self) -> &mut HashMap<String, Variable> {
        &mut self.variables
    }
}
