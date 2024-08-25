use instructions::{operand::BlkId, Instruction};
pub mod instructions;
use block::Block;
pub mod block;
use std::cell::{Ref, RefCell, RefMut};

pub struct IrObject<'a> {
    ext_fn : Vec<(String, u8, bool)>,
    variables : Vec<i64>,
    blocks : Vec<RefCell<Block<'a>>>
}

impl<'a> IrObject<'a> {
    pub fn new() -> Self {
        Self{
            ext_fn : Vec::new(),
            variables : Vec::new(),
            blocks : Vec::new()
        }
    }

    pub fn append_block(&mut self) -> BlkId{
        self.blocks.push(RefCell::new(Block::new()));
        return self.blocks.len() as BlkId
    }

    pub fn get_block<'b>(&'b self, id : BlkId) -> Ref<'b, Block<'a>> {
        self.blocks[id as usize].borrow()
    }

    pub fn get_block_mut<'b>(&'b self, id : BlkId) -> RefMut<'b, Block<'a>> {
        self.blocks[id as usize].borrow_mut()
    }
}
