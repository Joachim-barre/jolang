#[derive(Debug,Clone, PartialEq, Eq)]
pub enum Instructions {
    /// <
    Backward,
    /// >
    Forward,
    /// L
    Load,
    /// S
    Store,
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// P
    Print,
    /// ],
    Jump,
    /// }
    JumpIfZero,
    /// E
    Exit,
    /// I
    Inc,
    /// D
    Dec,
    /// C
    Compare
}
