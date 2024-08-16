use c_enum::c_enum;

c_enum! {
    #[derive(Clone, PartialEq, Eq)]
    pub enum Opcodes : u8{ 
        Exit = 0x00,
        Mkfr = 0x01,
        Delfr = 0x02,
        Pushi = 0x10,
        Pushv = 0x11,
        Pusht = 0x12,
        Br = 0x13,
        Call = 0x14,
        Neg = 0x15,
        Briz = 0x20,
        Store = 0x21,
        Add = 0x22,
        Sub = 0x23,
        Mul = 0x24,
        Div = 0x25,
        Eq = 0x26,
        Ne = 0x27,
        Gt = 0x28,
        Ge = 0x29,
        Le = 0x2A,
        Lt = 0x2B,
        Lsh = 0x2C,
        Rsh = 0x2D
    }
}
