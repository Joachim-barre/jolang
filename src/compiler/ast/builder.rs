use crate::compiler::lexer::{Lexer, LexerTokens};

pub struct AstBuilder<'a> {
    tokens : std::iter::Peekable<LexerTokens<'a>>
}

impl<'a> From<&'a mut Lexer> for AstBuilder<'a> {
    fn from(value: &'a mut Lexer) -> Self {
        Self {
            tokens : value.tokens().peekable()
        }
    }
}
