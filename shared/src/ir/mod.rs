use index_list::IndexList;
use instructions::{operand::{BlkId, FnId}, Instruction};
pub use signature::Signature;
pub mod instructions;
use std::cell::{Ref, RefCell, RefMut};
use crate::ffi::JolangExtern;
pub mod printer;
pub mod block;
pub mod writer;
pub mod reader;
pub mod signature;
use block::Block;

pub struct IrExternalFn {
    pub name : String,
    pub sig : Signature
}

pub struct IrObject {
    pub ext_fn : Vec<IrExternalFn>,
    pub blocks : Vec<Block>,
    pub local_vars : Vec<u8>
}

impl IrExternalFn {
    pub fn new(name : String, sig : Signature) -> Self{
        Self {
            name,
            sig
        }
    }
}

impl IrObject {
    pub fn new() -> Self {
        Self{
            ext_fn : Vec::new(),
            blocks : Vec::new(),
            local_vars : Vec::new()
        }
    }

    pub fn append_block(&mut self, args : Vec<u64>) -> BlkId{
        self.blocks.push(Block::new(args));
        return (self.blocks.len() as BlkId) -1
    }

    pub fn get_block<'b>(&'b self, id : BlkId) -> &Block {
        &self.blocks[id as usize]
    }

    pub fn get_block_mut<'b>(&'b mut self, id : BlkId) -> &mut Block {
        &mut self.blocks[id as usize]
    }

    pub fn decl_extern(&mut self, name : String, func : &Box<dyn JolangExtern>) -> FnId {
        self.ext_fn.push(IrExternalFn::new(name, func.signature()));
        (self.ext_fn.len() -1) as FnId
    }

    pub fn add_var(&mut self, size : u8) -> u32 {
        let tmp = self.local_vars.len();
        self.local_vars.push(size);
        tmp as u32
    } 
}
