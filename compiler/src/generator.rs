use jolang_shared::ir::{Block, instructions::{operand::{BlkId, VarId}, Instruction}, IrObject};
use std::cell::{Ref, RefMut};
use index_list::ListIndex;

pub struct IrGenerator {
    object : IrObject,
    current_pos : Option<(BlkId, ListIndex)>
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            object : IrObject::new(),
            current_pos : None
        }
    }

    pub fn decl_var(&mut self, value : i64) -> VarId {
        self.object.decl_var(value)
    }

    pub fn into_ir(self) -> IrObject{
        self.object
    }

    pub fn get_current_block<'b>(&'b self) -> Option<Ref<'b, Block>> {
        self.current_pos.as_ref().map(|id| self.object.get_block(id.0))
    }

    pub fn get_current_block_mut<'b>(&'b self) -> Option<RefMut<'b, Block>> {
        self.current_pos.as_ref().map(|id| self.object.get_block_mut(id.0))
    }

    pub fn add(&mut self, i : Instruction) -> Option<ListIndex> {
        self.current_pos.as_ref().map(|(_, pre)| {
            self.get_current_block_mut().unwrap().insert_after(*pre, i)
        })
    }

    pub fn append_block(&mut self) -> BlkId {
        self.object.append_block()
    }
    
    pub fn goto(&mut self, block : BlkId) {
        self.current_block = Some(block)
    }

    pub fn add_after(&mut self) {
        
    }
}

pub trait Generate {
    fn generate(&mut self, generator : &mut IrGenerator);
}
