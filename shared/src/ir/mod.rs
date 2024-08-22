use instructions::Instruction;
pub mod instructions;

struct IrObject<'a> {
    ext_fn : Vec<(String, u8, bool)>,
    variables : Vec<i64>,
    blocks : Vec<Vec<Instruction<'a>>>
}
