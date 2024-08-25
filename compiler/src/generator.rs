use jolang_shared::ir::{instructions::operand::BlkId, IrObject};

struct IrGenerator<'a> {
    object : IrObject<'a>,
    current_block : BlkId
}
