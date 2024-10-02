mod builder;
mod generators;
pub use builder::AstBuilder;

// TODO : Include SourceCursors into ast nodes to be able to generate error during IR generation

pub type Ident = String;

#[derive(Debug, PartialEq)]
pub struct Program (Vec<Statement>);

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(Vec<Statement>),
    /// (condition, then, else)
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    /// (condition, do)
    While(Expr, Box<Statement>),
    Loop(Box<Statement>),
    Return(Expr),
    Break,
    Continue,
    // type, name , value
    VarDecl(Option<Ident>, Ident, Option<Expr>),
    VarSet(Ident, Expr),
    Call(Call)
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    BinExpr(Box<Expr>, Box<Expr>, BinOp),
    UnaryExpr(UnaryOp, PrimaryExpr),
    PrimaryExpr(PrimaryExpr)
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
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

impl BinOp {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 2,
            Self::Equal => 0,
            Self::NotEqual => 0,
            Self::Greater => 0,
            Self::GreaterEqual => 0,
            Self::LesserEqual => 0,
            Self::Lesser => 0,
            Self::LShift => 0,
            Self::RShift => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus
}

#[derive(Debug, PartialEq)]
pub enum PrimaryExpr {
    Call(Call),
    Ident(Ident),
    Litteral(i128),
    /// (Expr) (e. g. (5 + 5))
    Expr(Box<Expr>)
}

#[derive(Debug, PartialEq)]
pub struct Call (Ident,Vec<Expr>);
