use c_enum::c_enum;

c_enum! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum Opcodes : u8{ 
        Ret = 0x00,
        Reti = 0x01,
        Iconst = 0x02,
        Br = 0x03,
        Dup = 0x04,
        Dupx = 0x05,
        Call = 0x05,
        Neg = 0x06,
        Add = 0x07,
        Sub = 0x08,
        Mul = 0x09,
        Div = 0x0a,
        Eq = 0x0b,
        Ne = 0x0c,
        Gt = 0x0d,
        Ge = 0x0e,
        Le = 0x0f,
        Lt = 0x10,
        Lsh = 0x11,
        Rsh = 0x12,
        Briz = 0x13
    }
}

pub mod operand {
    pub type Imm = i64;
    pub type BlkId = u64;
    pub type FnId = u64;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction {
    Ret(),
    Reti(),
    Iconst(operand::Imm),
    Br(operand::BlkId),
    Dup(),
    Dupx(operand::Imm),
    Call(operand::FnId),
    Neg(),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Eq(),
    Ne(),
    Gt(),
    Ge(),
    Le(),
    Lt(),
    Lsh(),
    Rsh(),
    Briz(operand::BlkId, operand::BlkId)
}

impl Instruction {
    pub fn opcode(&self) -> Opcodes {
        match self {
            Self::Ret() => Opcodes::Ret,
            Self::Reti() => Opcodes::Reti,
            Self::Iconst(_) => Opcodes::Iconst,
            Self::Br(_) => Opcodes::Br,
            Self::Dup() => Opcodes::Dup,
            Self::Dupx(_) => Opcodes::Dupx,
            Self::Call(_) => Opcodes::Call,
            Self::Neg() => Opcodes::Neg,
            Self::Add() => Opcodes::Add,
            Self::Sub() => Opcodes::Sub,
            Self::Mul() => Opcodes::Mul,
            Self::Div() => Opcodes::Div,
            Self::Eq() => Opcodes::Eq,
            Self::Ne() => Opcodes::Ne,
            Self::Gt() => Opcodes::Gt,
            Self::Ge() => Opcodes::Ge,
            Self::Le() => Opcodes::Le,
            Self::Lt() => Opcodes::Lt,
            Self::Lsh() => Opcodes::Lsh,
            Self::Rsh() => Opcodes::Rsh,
            Self::Briz(_, _) => Opcodes::Briz,
        }
    }
}
