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
        Swap = 0x06,
        Call = 0x07,
        Neg = 0x08,
        Add = 0x09,
        Sub = 0x0A,
        Mul = 0x0B,
        Div = 0x0C,
        Eq = 0x0D,
        Ne = 0x0E,
        Gt = 0x0F,
        Ge = 0x10,
        Le = 0x11,
        Lt = 0x12,
        Lsh = 0x13,
        Rsh = 0x14,
        Briz = 0x15
    }
}

pub mod operand {
    pub type Imm = i64;
    pub type UImm = u64;
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
    Dupx(operand::UImm),
    Swap(),
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
            Self::Swap() => Opcodes::Swap,
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
