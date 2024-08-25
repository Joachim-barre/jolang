use super::Instruction;

pub struct Block<'a> {
    pub instructions : Vec<Instruction<'a>>
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        Self {
            instructions : Vec::new()
        }
    }

    pub fn push(&mut self, i : Instruction<'a>) -> &'a Instruction<'a> {
        self.instructions.push(i);
        unsafe {
            std::mem::transmute(&self.instructions[self.instructions.len() - 1])
        }
    }
}
