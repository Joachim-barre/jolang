use super::instructions::Instruction;

#[derive(Debug)]
pub struct Block {
    pub argc : u8,
    pub instructions : Vec<Instruction>,
    // size of the stack at the end of the block (after the last instruction)
    pub stack_size : u64
}

impl Block {
    pub fn new(argc : u8) -> Self {
        Self {
            argc,
            instructions : Vec::new(),
            stack_size : argc as u64
        }
    }
}
