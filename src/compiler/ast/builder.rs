use crate::compiler::{compiler_error::{CompilerError, CompilerErrorKind},lexer::{KeywordType, Lexer, LexerTokens, Token, TokenKind}, source_buffer::SourceBuffer};
use super::{Program, Statement};
use std::{cell::RefCell, rc::Rc};

pub struct AstBuilder<'a> {
    tokens : LexerTokens<'a>,
    current : Option<Token<'a>>,
    source : Rc<RefCell<SourceBuffer>>
}

impl<'a> From<&'a mut Lexer> for AstBuilder<'a> {
    fn from(value: &'a mut Lexer) -> Self {
        let source = value.source.clone();
        Self {
            tokens : value.tokens(),
            current : None,
            source 
        }
    }
}

impl<'a> AstBuilder<'a> {
    pub fn unexpected(&self, token : &Token, name : Option<&str>) -> CompilerError {
        let msg = match name {
            Some(s) => format!("Unexpected token: {}", s),
            None => "Unexpected token".to_string()
        };
        CompilerError::new(
            CompilerErrorKind::UnexpectedToken,
            msg.as_str(),
            self.source.borrow().path.to_str().unwrap(),
            self.source.borrow().get_line(token.span.start.line).unwrap(),
            token.span.start.line as u32,
            token.span.start.collumn as u32,
            None)
    }

    pub fn expected(&self, name : &str) -> CompilerError {
        CompilerError::new(
            CompilerErrorKind::Expected,
            format!("Expected : {}", name).as_str(),
            self.source.borrow().path.to_str().unwrap(),
            self.source.borrow().get_line(self.tokens.lexer.pos.line).unwrap(),
            self.tokens.lexer.pos.line as u32,
            self.tokens.lexer.pos.collumn as u32,
            None)
    }

    pub fn peek_token(&self) -> &Option<Token> {
        &self.current
    }

    pub fn next_token(&mut self) -> Result<&Option<Token>, CompilerError> {
        match self.tokens.next() {
            Some(ret) => match ret {
                Ok(t) => self.current = Some(t),
                Err(e) => return Err(e)
            },
            None => return Ok(&None)
        }
        Ok(&self.current)
    }

    pub fn parse_program(&mut self) -> Result<Program, CompilerError>{
        let mut statments : Vec<Statement>= vec![];
        while self.next_token()?.is_some() {
            statments.push(self.parse_statment()?);
        }
        if statments.len() == 0 {
            return Err(self.expected("statement"))
        }
        Ok(Program ( statments ))
    }

    pub fn parse_statment(&mut self) -> Result<Statement, CompilerError>{
        let first_token = self.peek_token();
        let first_token = first_token.as_ref().unwrap();
        match &first_token.kind {
            crate::compiler::lexer::TokenKind::LCurly  => {
                let mut statements : Vec<Statement> = Vec::new();
                loop {
                    let token = self.next_token()?;
                    if !token.is_some() {
                        return Err(self.expected("\"}\""))
                    }
                    if token.as_ref().map(|x| x.kind == TokenKind::RCurly).unwrap() {
                        break;
                    }
                    statements.push(self.parse_statment()?)
                }
                Ok(Statement::Block(statements))
            },
            TokenKind::Keyword(k) => match k {
                KeywordType::If => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                        return Err(self.expected("\"(\""))
                    }
                    let cond = self.parse_expr()?;
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                        return Err(self.expected("\")\""))
                    }
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let then = Box::new(self.parse_statment()?);
                    let mut _else = None;
                    if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Keyword(KeywordType::Else)) {
                        _else = Some(Box::new(self.parse_statment()?));
                    }
                    Ok(Statement::If(cond, then, _else))
                }
                _ => todo!()
            }
            _ => Err(self.unexpected(first_token, None))
        }
    }

    pub fn parse_call(&mut self) -> Result<Call, CompilerError> {
        todo!();
    }
}
