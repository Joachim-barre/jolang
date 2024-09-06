use index_list::IndexList;
use super::instructions::Instruction;

#[derive(Debug, Default)]
pub struct Block {
    pub args : Vec<u64>,
    pub instructions : IndexList<Instruction>,
}

impl Block {
    pub fn new(args : Vec<u64>) -> Self {
        Self {
            args,
            instructions : IndexList::new()
        }
    }
}
