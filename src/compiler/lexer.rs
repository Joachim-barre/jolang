use super::source_span::SourceSpan;

pub enum TokenKind {
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
    Keyword(KeywordType),
    Ident
}

pub enum KeywordType {
    If,
    Else,
    While,
    Loop,
    Return,
    Break,
    Continue,
    Var
}

pub struct Token<'a> {
    kind : TokenKind,
    span : SourceSpan<'a>
}
