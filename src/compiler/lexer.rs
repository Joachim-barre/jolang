pub enum Tokens {
    LCurly,
    RCurly,
    LParan,
    RParan,
    Semicolon,
    Equal,
    DoubleEqual,
    NotEqual,
    Plus,
    Minus,
    Times,
    Divider,
    Greater,
    GreaterEqual,
    LesserEqual,
    Lesser,
    LShift,
    RShift,
    Comma,
    EOF,
    Keyword(Keywords),
    Ident
}

pub enum Keywords {
    If,
    Else,
    While,
    Loop,
    Return,
    Break,
    Continue,
    Var
}

struct Token {

}
