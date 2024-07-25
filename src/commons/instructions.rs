#[derive(Debug,Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Instructions {
    /// <
    Backward = 0,
    /// >
    Forward = 1,
    /// L
    Load = 2,
    /// S
    Store = 3,
    /// +
    Add = 4,
    /// -
    Sub = 5,
    /// *
    Mul = 6,
    /// /
    Div = 7,
    /// P
    Print = 8,
    /// ],
    Jump = 10,
    /// }
    JumpIfZero = 11,
    /// E
    Exit = 12,
    /// I
    Inc = 13,
    /// D
    Dec = 14,
    /// C
    Compare = 15
}
