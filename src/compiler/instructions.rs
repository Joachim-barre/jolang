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
    /// [
    Label,
    /// ],
    Jump,
    /// }
    JumpIfZero,
    /// Q
    Exit,
    /// I
    Inc,
    /// D
    Dec,
    /// C
    Compare
}
