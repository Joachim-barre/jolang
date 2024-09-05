use index_list::IndexList;
use instructions::{operand::{BlkId, FnId}, Instruction};
pub mod instructions;
use std::cell::{Ref, RefCell, RefMut};
use crate::ffi::JolangExtern;
pub mod printer;
pub mod block;
pub mod writer;
pub mod reader;
use block::Block;

pub struct IrObject {
    pub ext_fn : Vec<(String, u8, bool)>,
    pub blocks : Vec<Block>
}

impl IrObject {
    pub fn new() -> Self {
        Self{
            ext_fn : Vec::new(),
            blocks : Vec::new()
        }
    }

    pub fn append_block(&mut self, argc : u8) -> BlkId{
        self.blocks.push(Block::new(argc));
        return (self.blocks.len() as BlkId) -1
    }

    pub fn get_block<'b>(&'b self, id : BlkId) -> &Block {
        &self.blocks[id as usize]
    }

    pub fn get_block_mut<'b>(&'b mut self, id : BlkId) -> &mut Block {
        &mut self.blocks[id as usize]
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId {
        self.ext_fn.push((name, func.arg_count(), func.returns()));
        (self.ext_fn.len() -1) as FnId
    }
}
