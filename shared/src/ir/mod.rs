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
    blocks : Vec<RefCell<Block>>
}

impl IrObject {
    pub fn new() -> Self {
        Self{
            ext_fn : Vec::new(),
            blocks : Vec::new()
        }
    }

    pub fn append_block(&mut self, argc : u8) -> BlkId{
        self.blocks.push(RefCell::new(Block::new(argc)));
        return (self.blocks.len() as BlkId) -1
    }

    pub fn get_block<'b>(&'b self, id : BlkId) -> Ref<'b, Block> {
        self.blocks[id as usize].borrow()
    }

    pub fn get_block_mut<'b>(&'b self, id : BlkId) -> RefMut<'b, Block> {
        self.blocks[id as usize].borrow_mut()
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId {
        self.ext_fn.push((name, func.arg_count(), func.returns()));
        (self.ext_fn.len() -1) as FnId
    }
}
