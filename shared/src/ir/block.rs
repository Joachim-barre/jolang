use super::instructions::Instruction;

#[derive(Debug)]
pub struct Block {
    pub argc : u8,
    pub instructions : Vec<Instruction>
}

impl Block {
    pub fn new(argc : u8) -> Self {
        Self {
            argc,
            instructions : Vec::new()
        }
    }
}
