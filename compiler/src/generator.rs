use jolang_shared::ir::{instructions::operand::{BlkId, VarId}, IrObject};

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

    pub fn decl_var(&mut self, value : i64) {
        self.object.decl_var(value);
    }

    pub fn into_ir(self) -> IrObject<'a>{
        self.object
    }
}
