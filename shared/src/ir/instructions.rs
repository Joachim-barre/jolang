use c_enum::c_enum;

c_enum! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum Opcodes : u8{ 
        Ret = 0x00,
        Reti = 0x01,
        Iconst = 0x02,
        Icast = 0x03,
        Br = 0x04,
        Dup = 0x05,
        Dupx = 0x06,
        Swap = 0x07,
        Call = 0x08,
        Neg = 0x09,
        Add = 0x0A,
        Sub = 0x0B,
        Mul = 0x0C,
        Div = 0x0D,
        Eq = 0x0E,
        Ne = 0x0F,
        Gt = 0x10,
        Ge = 0x11,
        Le = 0x12,
        Lt = 0x13,
        Lsh = 0x14,
        Rsh = 0x15,
        Briz = 0x16
    }
}

pub mod operand {
    pub type Imm64 = i64;
    pub type UImm64 = u64;
    pub type UImm128 = i128;
    pub type BlkId = u64;
    pub type FnId = u64;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction {
    Ret(),
    Reti(),
    Iconst(operand::UImm64, operand::Imm64),
    Icast(operand::UImm64),
    Br(operand::BlkId),
    Dup(),
    Dupx(operand::UImm64),
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
            Self::Iconst(..) => Opcodes::Iconst,
            Self::Icast(..) => Opcodes::Icast,
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
