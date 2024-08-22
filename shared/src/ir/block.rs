use super::Instruction;

pub struct Block<'a> {
    pub instructions : Vec<Instruction<'a>>
}
