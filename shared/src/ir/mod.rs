use index_list::IndexList;
use instructions::{operand::{BlkId, FnId, VarId}, Instruction};
pub mod instructions;
use std::cell::{Ref, RefCell, RefMut};
use crate::ffi::JolangExtern;

pub type Block = IndexList<Instruction>;

#[derive(Debug)]
pub struct IrObject {
    pub ext_fn : Vec<(String, u8, bool)>,
    variables : Vec<i64>,
    blocks : Vec<RefCell<Block>>
}

impl IrObject {
    pub fn new() -> Self {
        Self{
            ext_fn : Vec::new(),
            variables : Vec::new(),
            blocks : Vec::new()
        }
    }

    pub fn append_block(&mut self) -> BlkId{
        self.blocks.push(RefCell::new(Block::new()));
        return (self.blocks.len() as BlkId) -1
    }

    pub fn get_block<'b>(&'b self, id : BlkId) -> Ref<'b, Block> {
        self.blocks[id as usize].borrow()
    }

    pub fn get_block_mut<'b>(&'b self, id : BlkId) -> RefMut<'b, Block> {
        self.blocks[id as usize].borrow_mut()
    }

    pub fn decl_var(&mut self, value : i64) -> VarId {
        self.variables.push(value);
        (self.variables.len() - 1) as VarId
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId {
        self.ext_fn.push((name, func.arg_count(), func.returns()));
        (self.ext_fn.len() -1) as FnId
    }
}
