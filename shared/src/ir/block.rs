use index_list::IndexList;
use super::instructions::Instruction;

#[derive(Debug, Default)]
pub struct Block {
    pub argc : u8,
    pub instructions : IndexList<Instruction>,
    // size of the stack at the end of the block (after the last instruction)
    pub stack_size : u64
}

impl Block {
    pub fn new(argc : u8) -> Self {
        Self {
            argc,
            instructions : IndexList::new(),
            stack_size : argc as u64
        }
    }
}
