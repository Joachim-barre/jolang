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
    use super::Instruction;

    pub type Imm = i64;
    pub type VarId = u64;
    pub type BlkId = u64;
    pub type Result<'a> = &'a Instruction<'a>;
    pub type FnId = u64;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction<'a> {
    Ret(),
    Reti(operand::Result<'a>),
    Varget(operand::VarId),
    Iconst(operand::Imm),
    Br(operand::BlkId),
    Pusharg(operand::Result<'a>),
    Call(operand::FnId),
    Neg(operand::Result<'a>),
    Varset(operand::VarId,operand::Result<'a>),
    Add(operand::Result<'a>,operand::Result<'a>),
    Sub(operand::Result<'a>,operand::Result<'a>),
    Mul(operand::Result<'a>,operand::Result<'a>),
    Div(operand::Result<'a>,operand::Result<'a>),
    Eq(operand::Result<'a>,operand::Result<'a>),
    Ne(operand::Result<'a>,operand::Result<'a>),
    Gt(operand::Result<'a>,operand::Result<'a>),
    Ge(operand::Result<'a>,operand::Result<'a>),
    Le(operand::Result<'a>,operand::Result<'a>),
    Lt(operand::Result<'a>,operand::Result<'a>),
    Lsh(operand::Result<'a>,operand::Result<'a>),
    Rsh(operand::Result<'a>,operand::Result<'a>),
    Briz(operand::BlkId,operand::BlkId,operand::Result<'a>)
}
