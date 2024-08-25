use std::collections::LinkedList;
use super::Instruction;

pub struct Block {
    pub instructions : LinkedList<Instruction>
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions : LinkedList::new()
        }
    }

    pub fn push<'b>(&'b mut self, i : Instruction) -> &'b Instruction {
        self.instructions.push_back(i);
        self.instructions.back().unwrap()
    }
}
