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

impl<'a> AstBuilder<'a> {
    pub fn parse_program(&mut self) -> Result<Program, CompilerError>{
        let mut statments : Vec<Statement>= vec![];
        while self.tokens.next().is_some() {
            statments.push(parse_statment());
        }
        if statments.len() == 0 {
            return Err(CompilerError::new(
                CompilerErrorKind::Expected,
                "expected stament",
                self.source.borrow().path.to_str().unwrap(),
                "",
                0,
                0,
                None
            ))
        }
        Ok(Program ( statments ))
    }
}
