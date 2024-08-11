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

    #[allow(unused_assignments)]
    pub fn parse_expr(&mut self) -> Result<Expr, CompilerError> {
        let mut unary_op = None;
        {
            let token = self.peek_token().as_ref().unwrap();
            // parse unary op
            unary_op = match &token.kind {
                TokenKind::Plus => Some(UnaryOp::Plus),
                TokenKind::Minus => Some(UnaryOp::Minus),
                _ => None
            };

            if unary_op.is_some() {
                { 
                    let _token = self.next_token()?;
                    match _token {
                        Some(_) => {},
                        None => return Err(self.expected("expression"))
                    }
                }
            }
        }
        let mut end_pos = SourcePos { line : 0, index : 0, collumn : 0};
        let mut _expr = None;
        {
            let token = self.peek_token().as_ref().unwrap();
            end_pos = token.span.end;
            let start_pos = token.span.start;

            // parse primary expression
            let primary = match &token.kind {
                TokenKind::Int => Ok(PrimaryExpr::Litteral(
                    match FromStr::from_str(token.span.data) {
                        Ok(i) => Ok(i),
                        Err(_) => {
                            return Err(CompilerError::new(
                                CompilerErrorKind::BadToken,
                                "cannot parse integer litteral",
                                self.source.borrow().path.to_str().unwrap(),
                                self.source.borrow().get_line(token.span.start.line).unwrap(),
                                token.span.start.line as u32,
                                token.span.start.collumn as u32,
                                None))
                        }
                    }?
                )),
                TokenKind::LParan => {
                        let sub_expr = self.parse_expr()?;
                        if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                            return Err(self.expected("\")\""))
                        }
                        Ok(PrimaryExpr::Expr(Box::new(sub_expr)))
                },
                TokenKind::Ident => {
                    let ident = Ident::from(token.span.data);
                    if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                        self.tokens.lexer.pos = start_pos;
                        Ok(PrimaryExpr::Call(self.parse_call()?))
                    }else {
                        self.tokens.lexer.pos = end_pos;
                        Ok(PrimaryExpr::Ident(ident))
                    }
                },
                _ => Err(self.unexpected(&token, None))
            }?;

            _expr = Some(match unary_op {
                Some(op) => Expr::UnaryExpr(op, primary),
                None => Expr::PrimaryExpr(primary)
            });
        }
        let expr = _expr.unwrap();
        let _token = self.next_token()?;
        if _token.is_none() {
            return Ok(expr)
        }

        let bin_op = match &_token.clone().unwrap().kind {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            TokenKind::Times => Some(BinOp::Mul),
            TokenKind::Divider => Some(BinOp::Div),
            TokenKind::DoubleEqual => Some(BinOp::Equal),
            TokenKind::NotEqual => Some(BinOp::NotEqual),
            TokenKind::Greater => Some(BinOp::Greater),
            TokenKind::GreaterEqual => Some(BinOp::GreaterEqual),
            TokenKind::LesserEqual => Some(BinOp::LesserEqual),
            TokenKind::Lesser => Some(BinOp::Lesser),
            TokenKind::LShift => Some(BinOp::LShift),
            TokenKind::RShift => Some(BinOp::RShift),
            _ => None
        };

        if bin_op.is_none() {
            self.tokens.lexer.pos = end_pos;
            return Ok(expr);
        }

        let bin_op = bin_op.unwrap();
        
        let _token = self.next_token()?;

        if _token.is_none() {
            return Err(self.expected("expr"))
        }

        let expr2 = self.parse_expr()?;
        Ok(self.apply_precedence(Expr::BinExpr(Box::new(expr), Box::new(expr2), bin_op)))
    }
}
