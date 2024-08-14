use crate::compiler::{compiler_error::{CompilerError, CompilerErrorKind},lexer::{KeywordType, Lexer, Token, TokenKind}, source_buffer::SourceBuffer};
use super::{Expr, Ident, PrimaryExpr, Program, Statement, UnaryOp, Call, BinOp};
use std::{cell::RefCell, rc::Rc, str::FromStr};

pub struct AstBuilder<'a> {
    lexer : Lexer<'a>,
    current : Option<Token<'a>>,
}

impl<'a> From<Lexer<'a>> for AstBuilder<'a> {
    fn from(value: Lexer<'a>) -> Self {
        Self {
            lexer : value,
            current : None,
        }
    }
}

impl<'a> AstBuilder<'a> {
    pub fn unexpected(&self, token : &Token) -> CompilerError {
        CompilerError::new(
            CompilerErrorKind::UnexpectedToken,
            format!("Unexpected token: {}", token.kind.to_str()).as_str(),
            token.span.source.path.to_str().unwrap(),
            token.span.source.get_line(token.span.start.line).unwrap(),
            token.span.start.line as u32,
            token.span.start.collumn as u32,
            None)
    }

    pub fn expected(&self, name : &str) -> CompilerError {
        CompilerError::new(
            CompilerErrorKind::Expected,
            format!("Expected : {}", name).as_str(),
            self.lexer.reader.source.path.to_str().unwrap(),
            self.lexer.reader.source.get_line(self.lexer.reader.current_cursor.line).unwrap(),
            self.lexer.reader.current_cursor.line as u32,
            self.lexer.reader.current_cursor.collumn as u32,
            None)
    }

    pub fn peek_token(&self) -> &Option<Token> {
        &self.current
    }

    pub fn next_token(&mut self) -> Result<&Option<Token>, CompilerError> {
        match self.lexer.next() {
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
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let cond = self.parse_expr()?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                        return Err(self.expected("\")\""))
                    }
                    if self.next_token()?.is_none() {
                        return Err(self.expected("statement"))
                    }
                    let then = Box::new(self.parse_statment()?);
                    let mut _else = None;
                    if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Keyword(KeywordType::Else)) {
                        if self.next_token()?.is_none() {
                            return Err(self.expected("statement"))
                        }
                        _else = Some(Box::new(self.parse_statment()?));
                    }
                    Ok(Statement::If(cond, then, _else))
                },
                KeywordType::While => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                        return Err(self.expected("\"(\""))
                    }
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let cond = self.parse_expr()?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                        return Err(self.expected("\")\""))
                    }
                    if self.next_token()?.is_none() {
                        return Err(self.expected("statement"))
                    }
                    let body = Box::new(self.parse_statment()?);
                    return Ok(Statement::While(cond, body))
                },
                KeywordType::Loop => {
                    if self.next_token()?.is_none() {
                        return Err(self.expected("statement"))
                    }
                    return Ok(Statement::Loop(Box::new(self.parse_statment()?)));
                },
                KeywordType::Return => {
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let value = self.parse_expr()?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::Return(value));

                },
                KeywordType::Break => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::Break);
                },
                KeywordType::Continue => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::Continue);
                },
                KeywordType::Var => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Ident) {
                        return Err(self.expected("identifier"))
                    }
                    let ident = Ident::from(self.peek_token().as_ref().unwrap().span.data); 
                    if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Equal) {
                        if self.next_token()?.is_none() {
                            return Err(self.expected("expression"))
                        }
                        let expr = self.parse_expr()?;
                        return Ok(Statement::VarDecl(ident, Some(expr)));
                    }else if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::VarDecl(ident, None))
                },
                _ => Err(self.unexpected(first_token))
            }
            _ => Err(self.unexpected(first_token))
        }
    }

    pub fn parse_call(&mut self) -> Result<Call, CompilerError> {
        let ident = Ident::from(self.next_token()?.as_ref().unwrap().span.data);
            if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                return Err(self.expected("\"(\""))
            }
            if self.next_token()?.is_none() {
                return Err(self.expected("\")\""))
            }else if self.peek_token().as_ref().unwrap().kind == TokenKind::RParan {
                return Ok(Call(ident, Vec::new()))
            }
            let mut args = vec![self.parse_expr()?];
            loop {
                if self.next_token()?.is_none() {
                    return Err(self.expected("\")\""))
                }else if self.peek_token().as_ref().unwrap().kind == TokenKind::RParan {
                    break;
                }
                args.push(self.parse_expr()?);
            }
            return Ok(Call(ident, args))
    }

    pub fn apply_precedence(&self, expr : Expr) -> Expr {
        match expr {
            // op1 is assumed to not be a bin op or to already match precedence
            // op2 is assumed to alrdy have precedence checked
            Expr::BinExpr(op1, op2, bin_op1) => {
                match *op2 {
                    Expr::BinExpr(op3, op4, bin_op2) => {
                        let prec1 = bin_op1.precedence();
                        let prec2 = bin_op2.precedence();

                        if prec1 > prec2 {
                            return Expr::BinExpr(Box::new(self.apply_precedence(Expr::BinExpr(op1, op3, bin_op1))), op4, bin_op2);
                        }
                        return Expr::BinExpr(op1, Box::new(Expr::BinExpr(op3, op4, bin_op2)), bin_op1)
                    },
                    _ => return Expr::BinExpr(op1, op2, bin_op1)
                }
            },
            _ => return expr 
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, CompilerError> {
        let token = self.peek_token().as_ref().unwrap();
        // parse unary op
        let unary_op = match &token.kind {
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
        let token = self.peek_token().as_ref().unwrap();
        let start_cursor = unsafe { std::mem::transmute(self.peek_token().as_ref().unwrap().span.start.clone()) };
        // parse primary expression
        let primary = match &token.kind {
            TokenKind::Int => Ok(PrimaryExpr::Litteral(
                match FromStr::from_str(token.span.data) {
                    Ok(i) => Ok(i),
                    Err(_) => {
                        return Err(CompilerError::new(
                            CompilerErrorKind::BadToken,
                            "cannot parse integer litteral",
                            token.span.source.path.to_str().unwrap(),
                            token.span.source.get_line(token.span.start.line).unwrap(),
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
                let end_pos = self.lexer.reader.current_cursor;
                if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                    self.lexer.reader.goto(start_cursor);
                    Ok(PrimaryExpr::Call(self.parse_call()?))
                }else {
                    self.lexer.reader.goto(end_pos);
                    Ok(PrimaryExpr::Ident(ident))
                }
            },
            _ => Err(self.unexpected(&token))
        }?;

        let expr = match unary_op {
            Some(op) => Expr::UnaryExpr(op, primary),
            None => Expr::PrimaryExpr(primary)
        };
        let _ = self.next_token()?;
        let _token = self.peek_token();
        if _token.is_none() {
            return Ok(expr)
        }

        let end_pos = self.lexer.reader.current_cursor;

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
            self.lexer.reader.goto(end_pos);
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
