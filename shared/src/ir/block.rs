use index_list::IndexList;
use super::instructions::Instruction;

#[derive(Debug, Default)]
pub struct Block {
    pub instructions : IndexList<Instruction>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions : IndexList::new()
        }
    }
}
