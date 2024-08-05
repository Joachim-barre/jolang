type Ident = String;

struct Program{
    statments : Vec<Statment>
}

enum Statment {
    Block(Vec<Statment>),
    /// (condition, then, else)
    If(Expr, Box<Statment>, Option<Box<Statment>>),
    /// (condition, do)
    While(Expr, Box<Statment>),
    Loop(Box<Statment>),
    Return(Expr),
    Break,
    Continue,
    VarDecl(Ident, Option<Expr>),
    VarSet(Ident, Expr),
    Call(Call)
}

enum Expr {
    BinExpr(Box<Expr>, Box<Expr>, BinOp),
    UnaryExpr(UnaryOp, Box<Expr>),
    PrimaryExpr(PrimaryExpr)
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    LesserEqual,
    Lesser,
    LShift,
    RShift
}

enum UnaryOp {
    Plus,
    Minus
}

enum PrimaryExpr {
    Call(Call),
    Ident(Ident),
    Litteral(i64),
    /// (Expr) (e. g. (5 + 5))
    Expr(Box<Expr>)
}

struct Call {
    id : Ident,
    args : Vec<Expr>
}
