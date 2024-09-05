use index_list::IndexList;
use super::instructions::Instruction;

#[derive(Debug, Default)]
pub struct Block {
    pub argc : u8,
    pub instructions : IndexList<Instruction>,
}

impl Block {
    pub fn new(argc : u8) -> Self {
        Self {
            argc,
            instructions : IndexList::new()
        }
    }
}
