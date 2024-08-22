use c_enum::c_enum;

c_enum! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum Opcodes : u8{ 
        Ret = 0x00,
        Reti = 0x10,
        Varget = 0x11,
        Iconst = 0x12,
        Br = 0x13,
        Pusharg = 0x14,
        Call = 0x15,
        Neg = 0x16,
        Varset = 0x20,
        Add = 0x21,
        Sub = 0x22,
        Mul = 0x23,
        Div = 0x24,
        Eq = 0x25,
        Ne = 0x26,
        Gt = 0x27,
        Ge = 0x28,
        Le = 0x29,
        Lt = 0x2a,
        Lsh = 0x2b,
        Rsh = 0x2c,
        Briz = 0x30
    }
}

pub mod operand {
    pub type Imm = i64;
    pub type VarId = u64;
    pub type BlkId = u64;
    pub type Result = u64;
    pub type FnId = u64;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction {
    Ret(),
    Reti(operand::Result),
    Varget(operand::VarId),
    Iconst(operand::Imm),
    Br(operand::BlkId),
    Pusharg(operand::Result),
    Call(operand::FnId),
    Neg(operand::Result),
    Varset(operand::VarId,operand::Result),
    Add(operand::Result,operand::Result),
    Sub(operand::Result,operand::Result),
    Mul(operand::Result,operand::Result),
    Div(operand::Result,operand::Result),
    Eq(operand::Result,operand::Result),
    Ne(operand::Result,operand::Result),
    Gt(operand::Result,operand::Result),
    Ge(operand::Result,operand::Result),
    Le(operand::Result,operand::Result),
    Lt(operand::Result,operand::Result),
    Lsh(operand::Result,operand::Result),
    Rsh(operand::Result,operand::Result),
    Briz(operand::BlkId,operand::BlkId,operand::Result)
}
