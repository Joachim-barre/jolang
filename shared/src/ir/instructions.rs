use c_enum::c_enum;

c_enum! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum Opcodes : u8{ 
        Nop = 0x00,
        Pop = 0x01,
        Dup = 0x02,
        Swap = 0x03,
        Br = 0x04,
        Briz = 0x05,
        Call = 0x06,
        Varref = 0x07,
        Iconst = 0x08,
        Iload = 0x09,
        Istore = 0x0A,
        Iret = 0x0B,
        Inot = 0x0C,
        Ior = 0x0D,
        Iand = 0x0E,
        Ixor = 0x0F,
        Ilshr = 0x10,
        Iashr = 0x11,
        Ishl = 0x12,
        Ineg = 0x13,
        Iadd = 0x14,
        Isub = 0x15,
        Imul = 0x16,
        Idiv = 0x17,
        Udiv = 0x18,
        Irem = 0x19,
        Urem = 0x1A,
        Ieq = 0x1B,
        Ine = 0x1C,
        Ige = 0x1D,
        Igt = 0x1E,
        Uge = 0x1F,
        Ugt = 0x20,
        Ilt = 0x21,
        Ile = 0x22,
        Ule = 0x23,
        Ult = 0x24,
        Iconv = 0x25,
        Uconv = 0x26
    }
}

pub mod operand {
    pub type Imm = i64;
    pub type BlkId = u32;
    pub type FnId = u32;
    pub type Size = u32;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction {
    Nop(),
    Pop(operand::Size),
    Dup(operand::Size),
    Swap(operand::Size),
    Br(operand::BlkId),
    Briz(operand::BlkId, operand::BlkId),
    Call(operand::FnId),
    Varref(),
    Iconst(operand::Size, operand::Imm),
    Iload(operand::Size),
    Istore(operand::Size),
    Iret(operand::Size),
    Inot(operand::Size),
    Ior(operand::Size),
    Iand(operand::Size),
    Ixor(operand::Size),
    Ilshr(operand::Size),
    Iashr(operand::Size),
    Ishl(operand::Size),
    Ineg(operand::Size),
    Iadd(operand::Size),
    Isub(operand::Size),
    Imul(operand::Size),
    Idiv(operand::Size),
    Udiv(operand::Size),
    Irem(operand::Size),
    Urem(operand::Size),
    Ieq(operand::Size),
    Ine(operand::Size),
    Ige(operand::Size),
    Igt(operand::Size),
    Uge(operand::Size),
    Ugt(operand::Size),
    Ilt(operand::Size),
    Ile(operand::Size),
    Ule(operand::Size),
    Ult(operand::Size),
    Iconv(operand::Size, operand::Size),
    Uconv(operand::Size, operand::Size)
}

impl Instruction {
    pub fn opcode(&self) -> Opcodes {
        match self {
            Self::Nop(..) => Opcodes::Nop,
            Self::Pop(..) => Opcodes::Pop,
            Self::Dup(..) => Opcodes::Dup,
            Self::Swap(..) => Opcodes::Swap,
            Self::Br(..) => Opcodes::Br,
            Self::Briz(..) => Opcodes::Briz,
            Self::Call(..) => Opcodes::Call,
            Self::Varref(..) => Opcodes::Varref,
            Self::Iconst(..) => Opcodes::Iconst,
            Self::Iload(..) => Opcodes::Iload,
            Self::Istore(..) => Opcodes::Istore,
            Self::Iret(..) => Opcodes::Iret,
            Self::Inot(..) => Opcodes::Inot,
            Self::Ior(..) => Opcodes::Ior,
            Self::Iand(..) => Opcodes::Iand,
            Self::Ixor(..) => Opcodes::Ixor,
            Self::Ilshr(..) => Opcodes::Ilshr,
            Self::Iashr(..) => Opcodes::Iashr,
            Self::Ishl(..) => Opcodes::Ishl,
            Self::Ineg(..) => Opcodes::Ineg,
            Self::Iadd(..) => Opcodes::Iadd,
            Self::Isub(..) => Opcodes::Isub,
            Self::Imul(..) => Opcodes::Imul,
            Self::Idiv(..) => Opcodes::Idiv,
            Self::Udiv(..) => Opcodes::Udiv,
            Self::Irem(..) => Opcodes::Irem,
            Self::Urem(..) => Opcodes::Urem,
            Self::Ieq(..) => Opcodes::Ieq,
            Self::Ine(..) => Opcodes::Ine,
            Self::Ige(..) => Opcodes::Ige,
            Self::Igt(..) => Opcodes::Igt,
            Self::Uge(..) => Opcodes::Uge,
            Self::Ugt(..) => Opcodes::Ugt,
            Self::Ilt(..) => Opcodes::Ilt,
            Self::Ile(..) => Opcodes::Ile,
            Self::Ule(..) => Opcodes::Ule,
            Self::Ult(..) => Opcodes::Ult,
            Self::Iconv(..) => Opcodes::Iconv,
            Self::Uconv(..) => Opcodes::Uconv
        }
    }
}
