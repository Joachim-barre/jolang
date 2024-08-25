use jolang_shared::ir::{block::Block, instructions::{operand::{BlkId, VarId}, Instruction}, IrObject};
use std::cell::{Ref, RefMut};

struct IrGenerator<'a> {
    object : IrObject<'a>,
    current_block : Option<BlkId>
}

impl<'a> IrGenerator<'a> {
    pub fn new() -> Self {
        Self {
            object : IrObject::new(),
            current_block : None
        }
    }

    pub fn decl_var(&mut self, value : i64) {
        self.object.decl_var(value);
    }

    pub fn into_ir(self) -> IrObject<'a>{
        self.object
    }

    pub fn get_current_block<'b>(&'b self) -> Option<Ref<'b, Block<'a>>> {
        self.current_block.as_ref().map(|id| self.object.get_block(*id))
    }

    pub fn get_current_block_mut<'b>(&'b self) -> Option<RefMut<'b, Block<'a>>> {
        self.current_block.as_ref().map(|id| self.object.get_block_mut(*id))
    }

    pub fn append(&mut self, i : Instruction<'a>) -> Option<&Instruction> {
        match self.get_current_block_mut() {
            Some(mut blk) => {
                Some(unsafe { std::mem::transmute(blk.push(i)) })
            },
            None => None
        }
    }

    pub fn append_block(&mut self) -> BlkId {
        self.object.append_block()
    }
}
