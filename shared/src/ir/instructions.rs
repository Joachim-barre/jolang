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
    pub type Offset = u64;
    pub type FnId = u64;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction {
	Exit(),
	Mkfr(),
	Delfr(),
	Pushi(operand::Imm),
	Pushv(operand::VarId),
	Pusht(operand::Offset),
	Br(operand::BlkId),
	Call(operand::FnId),
	Neg(operand::Offset),
	Briz(operand::BlkId, operand::Offset),
	Store(operand::VarId, operand::Offset),
	Add(operand::Offset,operand::Offset),
	Sub(operand::Offset,operand::Offset),
	Mul(operand::Offset,operand::Offset),
	Div(operand::Offset,operand::Offset),
	Eq(operand::Offset,operand::Offset),
	Ne(operand::Offset,operand::Offset),
	Gt(operand::Offset,operand::Offset),
	Ge(operand::Offset,operand::Offset),
	Le(operand::Offset,operand::Offset),
	Lt(operand::Offset,operand::Offset),
	Lsh(operand::Offset,operand::Offset),
	Rsh(operand::Offset,operand::Offset),
}
