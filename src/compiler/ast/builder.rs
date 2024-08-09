use crate::compiler::{compiler_error::{CompilerError, CompilerErrorKind},lexer::{Lexer, LexerTokens}, source_buffer::SourceBuffer};
use super::{Program, Statement};
use std::{cell::RefCell, rc::Rc};

pub struct AstBuilder<'a> {
    tokens : std::iter::Peekable<LexerTokens<'a>>,
    source : Rc<RefCell<SourceBuffer>>
}

impl<'a> From<&'a mut Lexer> for AstBuilder<'a> {
    fn from(value: &'a mut Lexer) -> Self {
        let source = value.source.clone();
        Self {
            tokens : value.tokens().peekable(),
            source 
        }
    }
}
