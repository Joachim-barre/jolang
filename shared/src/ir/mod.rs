use instructions::Instruction;
pub mod instructions;
use block::Block;
pub mod block;

pub struct IrObject<'a> {
    ext_fn : Vec<(String, u8, bool)>,
    variables : Vec<i64>,
    blocks : Vec<Block<'a>>
}

impl<'a> IrObject<'a> {
    pub fn new() -> Self {
        Self{
            ext_fn : Vec::new(),
            variables : Vec::new(),
            blocks : Vec::new()
        }
    }
}
