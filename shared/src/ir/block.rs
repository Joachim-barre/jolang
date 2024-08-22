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
}
