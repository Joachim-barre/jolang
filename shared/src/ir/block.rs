use std::collections::LinkedList;
use super::Instruction;

pub struct Block<'a> {
    pub instructions : LinkedList<Instruction<'a>>
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        Self {
            instructions : LinkedList::new()
        }
    }

    pub fn push<'b>(&'b mut self, i : Instruction<'a>) -> &'b Instruction<'a> {
        self.instructions.push_back(i);
        self.instructions.back().unwrap()
    }
}
