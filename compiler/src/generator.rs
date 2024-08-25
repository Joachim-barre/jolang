use jolang_shared::ir::{instructions::operand::BlkId, IrObject};

struct IrGenerator<'a> {
    object : IrObject<'a>,
    current_block : Option<BlkId>
}

impl<'a> IrGenerator<'a> {
    pub fn new() -> Self {
        Self {
            object : IrObject::new(),
            current_block : None
        }
    }
}
