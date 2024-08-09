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
            statments.push(self.parse_statment()?);
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

    pub fn parse_statment(&mut self) -> Result<Statement, CompilerError>{
        let first_token = &self.tokens.peek().unwrap();
        if let Err(e) = first_token {
            return Err((*e).clone())
        }
        let first_token = first_token.as_ref().unwrap();
        match first_token.kind {
            _ => Err(CompilerError::new(
                    CompilerErrorKind::UnexpectedToken,
                    "Unexpected token",
                    self.source.borrow().path.to_str().unwrap(),
                    self.source.borrow().get_line(first_token.span.start.line).unwrap(),
                    first_token.span.start.line as u32,
                    first_token.span.start.collumn as u32,
                    None))
        }
    }
}
