/// block that are not completly built yet
use index_list::IndexList;
use jolang_shared::ir::instructions::Instruction;

#[derive(Debug, Default)]
pub struct Block {
    // args size
    pub args : Vec<u64>,
    pub instructions : IndexList<Instruction>,
    // size of the stack at the end of the block (after the last instruction)
    pub stack_types : Vec<u64>
}

impl Block {
    pub fn new(args : Vec<u64>) -> Self {
        Self {
            args : args.clone(),
            instructions : IndexList::new(),
            stack_types : args.clone()
        }
    }

    pub fn into_ir_block(self) -> jolang_shared::ir::block::Block {
        jolang_shared::ir::block::Block {
            args : self.args,
            instructions : self.instructions
        }
    }

    pub fn stack_size(&self) -> u64 {
        self.stack_types.len() as u64
    }
}
