use either::Either;
use crate::{compiler_error::{CompilerError, CompilerErrorKind},lexer::{KeywordType, Lexer, Token, TokenKind}, source_buffer::SourceBuffer, source_reader::SourceCursor};
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
            format!("Unexpected token: {} (\"{}\")", token.kind.to_str(), token.span.data).as_str(),
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

    pub fn peek_token(&self) -> &Option<Token<'a>> {
        &self.current
    }

    pub fn next_token(&mut self) -> Result<&Option<Token<'a>>, CompilerError> {
        match self.lexer.next() {
            Some(ret) => match ret {
                Ok(t) => self.current = Some(t),
                Err(e) => return Err(e)
            },
            None => return Ok(&None)
        }
        Ok(&self.current)
    }

    pub fn parse_program(&mut self) -> Result<Program<'a>, CompilerError>{
        let mut statments : Vec<Statement>= vec![];
        if self.next_token()?.is_none() {
            return Err(self.expected("statement"))
        }
        loop {
            statments.push(self.parse_statment()?);
            if self.next_token()?.is_none(){
                break;
            }
        }
        Ok(Program ( statments ))
    }

    pub fn parse_statment(&mut self) -> Result<Statement<'a>, CompilerError>{
        let first_token = self.peek_token().clone();
        let first_token = first_token.as_ref().unwrap();
        let start_cursor = unsafe { std::mem::transmute(self.peek_token().as_ref().unwrap().span.start.clone()) };
        match &first_token.kind {
            TokenKind::Keyword(k) => match k {
                KeywordType::Return => {
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let value = self.parse_expr()?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::Return(super::Return {
                        return_kw : first_token.clone(),
                        value,
                        semicolon : self.peek_token().as_ref().unwrap().clone()
                    }));

                },
                KeywordType::Break => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::Break(super::Break {
                        break_kw: first_token.clone(),
                        semicolon: self.peek_token().as_ref().unwrap().clone() 
                    }));
                },
                KeywordType::Continue => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::Continue(super::Continue {
                        continue_kw: first_token.clone(),
                        semicolon: self.peek_token().as_ref().unwrap().clone() 
                    }));
                },
                KeywordType::Let => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Ident) {
                        return Err(self.expected("identifier"))
                    }
                    let ident = Ident::from(self.peek_token().as_ref().unwrap().clone()); 
                    let _type = if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Colon) {
                        let colon_token = self.peek_token().as_ref().unwrap().clone();
                        if self.next_token()?.as_ref().map_or(false, |x| x.kind != TokenKind::Ident) {
                            return Err(self.expected("identifier"))
                        }                      
                        let _type = self.peek_token().as_ref().unwrap().clone();
                        self.next_token()?;
                        Ok(Some((colon_token, _type)))
                    }else {
                        Ok(None)
                    }?;
                    let val = if self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Equal) {
                        let eq_token = self.peek_token().as_ref().unwrap().clone();
                        if self.next_token()?.is_none() {
                            return Err(self.expected("expression"))
                        }
                        let expr = self.parse_expr()?;
                        Ok(Some((eq_token, expr)))
                    }else {
                        Ok(None)
                    }?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::VarDecl(super::VarDecl { 
                        let_kw: first_token.clone(),
                        name: ident,
                        colon_token : _type.as_ref().map(|x| x.0.clone()),
                        type_name : _type.as_ref().map(|x| x.1.clone()),
                        eq_token: val.as_ref().map(|v| v.0.clone()),
                        value: val.as_ref().map(|v| v.1.clone()),
                        semicolon: self.peek_token().as_ref().unwrap().clone()
                    }))
                },
                _ => Err(self.unexpected(first_token))
            },
            TokenKind::Ident => {
                let ident = first_token.clone();
                if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                    self.lexer.reader.goto(start_cursor);
                    let call = self.parse_call()?;
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    Ok(Statement::Call(call))
                }else {
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Equal) {
                        return Err(self.unexpected(first_token))
                    }
                    let eq_token = self.peek_token().as_ref().unwrap().clone();
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expression"))
                    }
                    let expr = self.parse_expr()?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::Semicolon) {
                        return Err(self.expected("\";\""))
                    }
                    return Ok(Statement::VarSet(super::VarSet {
                        name: ident,
                        eq_token,
                        value: expr,
                        semicolon: self.peek_token().as_ref().unwrap().clone() 
                    }));
                }
            },
            _ => Err(self.unexpected(first_token))
        }
    }

    pub fn parse_call(&mut self) -> Result<Call<'a>, CompilerError> {
        let ident = self.next_token()?.as_ref().unwrap().clone();
            if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                return Err(self.expected("\"(\""))
            }
            let lparen = self.peek_token().as_ref().unwrap().clone();
            if self.next_token()?.is_none() {
                return Err(self.expected("\")\""))
            }else if self.peek_token().as_ref().unwrap().kind == TokenKind::RParan {
                return Ok(Call{
                    name : ident,
                    lparen,
                    first_arg : None,
                    other_args : vec![],
                    rparen : self.peek_token().as_ref().unwrap().clone()
                })
            }
            let first_arg = Some(Box::new(self.parse_expr()?));
            let mut other_args = vec![];
            loop {
                if self.peek_token().is_none() {
                    return Err(self.expected("\")\""))
                }else if self.peek_token().as_ref().unwrap().kind == TokenKind::RParan {
                    break;
                }else if !(self.peek_token().as_ref().unwrap().kind == TokenKind::Comma) {
                    return Err(self.expected("\",\""))
                }
                let comma = self.peek_token().as_ref().unwrap().clone();
                self.next_token()?;
                other_args.push((comma, self.parse_expr()?));
            }
            return Ok(Call{
                name : ident,
                lparen,
                first_arg,
                other_args,
                rparen : self.peek_token().as_ref().unwrap().clone()
            })
    }

    pub fn apply_precedence<'b>(&self, expr : Expr<'b>) -> Expr<'b> {
        match expr {
            // op1 is assumed to not be a bin op or to already match precedence
            // op2 is assumed to alrdy have precedence checked
            Expr::BinExpr(expr1) => {
                match *expr1.right {
                    Expr::BinExpr(expr2) => {
                        let prec1 = expr1.op.kind.precedence();
                        let prec2 = expr2.op.kind.precedence();

                        if prec1 > prec2 {
                            return Expr::BinExpr(super::BinExpr {
                                left : Box::new(Expr::BinExpr(super::BinExpr {
                                    left : expr1.left,
                                    right : expr2.left,
                                    op : expr1.op
                                })),
                                right : expr2.right,
                                op : expr2.op
                            });
                        }
                        return Expr::BinExpr(super::BinExpr {
                            left : expr1.left,
                            right : Box::new(Expr::BinExpr(expr2)),
                            op : expr1.op
                        })
                    },
                    _ => return Expr::BinExpr(expr1)
                }
            },
            _ => return expr 
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr<'a>, CompilerError> {
        let token = self.peek_token().as_ref().unwrap().clone();
        match &token.kind {
            TokenKind::LCurly  => {
                let mut statements : Vec<Statement> = Vec::new();
                let lcurly = token.clone();
                loop {
                    let token = self.next_token()?;
                    if !token.is_some() {
                        return Err(self.expected("\"}\""))
                    }
                    if token.as_ref().map(|x| x.kind == TokenKind::RCurly).unwrap() {
                        break;
                    }
                    let current_cursor  : SourceCursor<'a> = unsafe { std::mem::transmute(self.peek_token().as_ref().unwrap().span.start.clone()) };
                    if let Ok(expr) = self.parse_expr() {
                        let cursor2 : SourceCursor<'a> = unsafe { std::mem::transmute(self.peek_token().as_ref().unwrap().span.start.clone()) };
                        if self.next_token()?.as_ref().map_or(false, |t| t.kind == TokenKind::RCurly){
                            return Ok(Expr::BlockExpr(super::Block { 
                                lcurly,
                                body: statements,
                                ret : Some(Box::new(expr)),
                                rcurly: self.peek_token().as_ref().unwrap().clone()
                            }))
                        }else {
                            let semicolon = if expr.require_semicolon(){
                                if self.peek_token().as_ref().map_or(false, |t| t.kind != TokenKind::Semicolon) {
                                    return Err(self.expected("\";\""));
                                }
                                Some(self.peek_token().as_ref().unwrap().clone())
                            }else {
                                self.lexer.reader.goto(cursor2);
                                self.next_token()?;
                                None
                            };
                            statements.push(Statement::Expr(super::ExprStmt { 
                                expr : Box::new(expr), 
                                semicolon
                            }))
                        }
                    }else {
                        self.lexer.reader.goto(current_cursor);
                        statements.push(self.parse_statment()?);
                    }
                }
                Ok(Expr::BlockExpr(super::Block { 
                    lcurly,
                    body: statements,
                    ret : None,
                    rcurly: self.peek_token().as_ref().unwrap().clone()
                }))
            },
            TokenKind::Keyword(kw) => match kw {
                KeywordType::If => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                        return Err(self.expected("\"(\""))
                    }
                    let lparen = self.peek_token().as_ref().unwrap().clone();
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let cond = Box::new(self.parse_expr()?.clone());
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                        return Err(self.expected("\")\""))
                    }
                    let rparen = self.peek_token().as_ref().unwrap().clone();
                    if self.next_token()?.is_none() {
                        return Err(self.expected("statement"))
                    }
                    let then = Box::new(self.parse_expr()?);
                    let mut else_kw = None;
                    let mut _else = None;
                    let cursor = self.lexer.reader.current_cursor.clone();
                    if self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::Keyword(KeywordType::Else)) {
                        else_kw = self.peek_token().clone();
                        if self.next_token()?.is_none() {
                            return Err(self.expected("statement"))
                        }
                        _else = Some(Box::new(self.parse_expr()?));
                    }else {
                        self.lexer.reader.goto(cursor);
                    }
                    Ok(Expr::IfExpr(super::If {
                        if_kw: token.clone(),
                        lparen,
                        cond,
                        rparen,
                        then,
                        else_kw,
                        _else
                    }))
                },
                KeywordType::While => {
                    if !self.next_token()?.as_ref().map_or(false, |x| x.kind == TokenKind::LParan) {
                        return Err(self.expected("\"(\""))
                    }
                    let lparen = self.peek_token().as_ref().unwrap().clone();
                    if self.next_token()?.is_none() {
                        return Err(self.expected("expr"))
                    }
                    let cond = Box::new(self.parse_expr()?);
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                        return Err(self.expected("\")\""))
                    }
                    let rparen = self.peek_token().as_ref().unwrap().clone();
                    if self.next_token()?.is_none() {
                        return Err(self.expected("statement"))
                    }
                    let body = Box::new(self.parse_expr()?);
                    return Ok(Expr::WhileExpr(super::While {
                        while_kw: token.clone(),
                        lparen,
                        cond,
                        rparen,
                        body
                    }))
                },
                KeywordType::Loop => {
                    if self.next_token()?.is_none() {
                        return Err(self.expected("statement"))
                    }
                    return Ok(Expr::LoopExpr(super::Loop {
                        loop_kw : token.clone(),
                        body : Box::new(self.parse_expr()?)
                    }));
                },
                _ => self.parse_arithmetic_expr()
            },
            _ => self.parse_arithmetic_expr()
        }
    }

    pub fn parse_arithmetic_expr(&mut self) -> Result<Expr<'a>, CompilerError> {
        let token = self.peek_token().as_ref().unwrap();
        // parse unary op
        let unary_op = match &token.kind {
            TokenKind::Plus => Some(UnaryOp{
                token: token.clone(),
                kind: super::UnaryOpKind::Plus
            }),
            TokenKind::Minus => Some(UnaryOp{
                token: token.clone(),
                kind: super::UnaryOpKind::Minus
            }),
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
            TokenKind::Int => Ok(PrimaryExpr::IntLit(
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
                    let lparen = token.clone();
                    self.next_token()?;
                    let sub_expr = self.parse_expr()?;
                    if !self.peek_token().as_ref().map_or(false, |x| x.kind == TokenKind::RParan) {
                        return Err(self.expected("\")\""))
                    }
                    Ok(PrimaryExpr::Paren(super::ParenExpr {
                        lparen,
                        expr : Box::new(sub_expr),
                        rparen : self.peek_token().as_ref().unwrap().clone()
                    }))
            },
            TokenKind::Ident => {
                let ident = token.clone();
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
            Some(op) => Expr::UnaryExpr(super::UnaryExpr { 
                primary, 
                op
            }),
            None => Expr::PrimaryExpr(primary)
        };
        let _ = self.next_token()?;
        let _token = self.peek_token();
        if _token.is_none() {
            return Ok(expr)
        }

        let end_pos = self.lexer.reader.current_cursor;

        let bin_op_kind = match &_token.clone().unwrap().kind {
            TokenKind::Plus => Some(super::BinOpKind::Add),
            TokenKind::Minus => Some(super::BinOpKind::Sub),
            TokenKind::Times => Some(super::BinOpKind::Mul),
            TokenKind::Divider => Some(super::BinOpKind::Div),
            TokenKind::DoubleEqual => Some(super::BinOpKind::Equal),
            TokenKind::NotEqual => Some(super::BinOpKind::NotEqual),
            TokenKind::Greater => Some(super::BinOpKind::Greater),
            TokenKind::GreaterEqual => Some(super::BinOpKind::GreaterEqual),
            TokenKind::LesserEqual => Some(super::BinOpKind::LesserEqual),
            TokenKind::Lesser => Some(super::BinOpKind::Lesser),
            TokenKind::LShift => Some(super::BinOpKind::LShift),
            TokenKind::RShift => Some(super::BinOpKind::RShift),
            _ => None
        };

        if bin_op_kind.is_none() {
            self.lexer.reader.goto(end_pos);
            return Ok(expr);
        }

        let bin_op_kind = bin_op_kind.unwrap();
        
        let bin_op = BinOp {
            token : _token.clone().unwrap(),
            kind : bin_op_kind
        };

        let _token = self.next_token()?;

        if _token.is_none() {
            return Err(self.expected("expr"))
        }

        let expr2 = self.parse_expr()?;
        Ok(self.apply_precedence(Expr::BinExpr(super::BinExpr {
            left: Box::new(expr),
            right: Box::new(expr2),
            op: bin_op
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, source_reader::SourceCursor, source_span::SourceSpan};
    use pretty_assertions::assert_eq;
    use core::panic;
    use std::path::PathBuf;

    #[test]
    fn test_var_decl() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("var n;\ni128 _m6 = 1;")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::VarDecl(VarDecl { 
                            type_var_kw: Either::Right(
                                Token { 
                                    kind: TokenKind::Keyword(KeywordType::Var),
                                    span: SourceSpan { 
                                        start: SourceCursor {
                                            line : 1,
                                            collumn : 1,
                                            data_ref : ""
                                        },
                                        size: 3,
                                        data: "var",
                                        source: &buf
                                    }
                                }
                            ), 
                            name: Token { 
                                kind: TokenKind::Ident,
                                span: SourceSpan { 
                                    start: SourceCursor {
                                        line : 1,
                                        collumn : 5,
                                        data_ref : ""
                                    },
                                    size: 1,
                                    data: "n",
                                    source: &buf
                                }
                            },
                            eq_token: None,
                            value: None,
                            semicolon: Token { 
                                kind: TokenKind::Semicolon,
                                span: SourceSpan { 
                                    start: SourceCursor {
                                        line : 1,
                                        collumn : 6,
                                        data_ref : ""
                                    },
                                    size: 1,
                                    data: ";",
                                    source: &buf
                                }
                            },
                        }),
                        Statement::VarDecl(VarDecl { 
                            type_var_kw: Either::Left(
                                Token { 
                                    kind: TokenKind::Ident,
                                    span: SourceSpan { 
                                        start: SourceCursor {
                                            line : 2,
                                            collumn : 1,
                                            data_ref : ""
                                        },
                                        size: 4,
                                        data: "i128",
                                        source: &buf
                                    }
                                }
                            ), 
                            name: Token { 
                                kind: TokenKind::Ident,
                                span: SourceSpan { 
                                    start: SourceCursor {
                                        line : 2,
                                        collumn : 6,
                                        data_ref : ""
                                    },
                                    size: 3,
                                    data: "_m6",
                                    source: &buf
                                }
                            },
                            eq_token: Some(Token { 
                                kind: TokenKind::Equal,
                                span: SourceSpan { 
                                    start: SourceCursor {
                                        line : 2,
                                        collumn : 10,
                                        data_ref : ""
                                    },
                                    size: 1,
                                    data: "=",
                                    source: &buf
                                }
                            }),
                            value: Some(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                            semicolon: Token { 
                                kind: TokenKind::Semicolon,
                                span: SourceSpan { 
                                    start: SourceCursor {
                                        line : 2,
                                        collumn : 13,
                                        data_ref : ""
                                    },
                                    size: 1,
                                    data: ";",
                                    source: &buf
                                }
                            },
                        })
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }
    
    #[test]
    fn test_return_break_continue() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("return 0;\nbreak;continue;")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::Return(Return { 
                            return_kw: Token { 
                                kind: TokenKind::Keyword(KeywordType::Return),
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 1
                                    },
                                    size : 6,
                                    data : "return",
                                    source : &buf
                                }
                            },
                            value: Expr::PrimaryExpr(PrimaryExpr::Litteral(0)),
                            semicolon: Token { 
                                kind: TokenKind::Semicolon,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 9
                                    },
                                    size : 1,
                                    data : ";",
                                    source : &buf
                                }
                            } 
                        }),
                        Statement::Break(Break {
                            break_kw : Token { 
                                kind: TokenKind::Keyword(KeywordType::Break),
                                span: SourceSpan {
                                    data : "break",
                                    size : 5,
                                    source : &buf,
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 2,
                                        collumn : 1
                                    }
                                }
                            },
                            semicolon: Token { 
                                kind: TokenKind::Semicolon,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 2,
                                        collumn : 6
                                    },
                                    size : 1,
                                    data : ";",
                                    source : &buf
                                }
                            } 
                        }),
                        Statement::Continue(Continue {
                            continue_kw : Token { 
                                kind: TokenKind::Keyword(KeywordType::Continue),
                                span: SourceSpan {
                                    data : "continue",
                                    size : 8,
                                    source : &buf,
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 2,
                                        collumn : 7
                                    }
                                }
                            },
                            semicolon: Token { 
                                kind: TokenKind::Semicolon,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 2,
                                        collumn : 15
                                    },
                                    size : 1,
                                    data : ";",
                                    source : &buf
                                }
                            } 
                        })
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_if_else() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("if (1) {} if (0) { return 1; } else return 0;")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::If(If {
                            if_kw: Token { 
                                kind: TokenKind::Keyword(KeywordType::If),
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 1,
                                    },
                                    size : 2,
                                    data : "if",
                                    source : &buf
                                }
                            },
                            lparen: Token { 
                                kind: TokenKind::LParan,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 4,
                                    },
                                    size : 1,
                                    data : "(",
                                    source : &buf
                                }
                            },
                            cond: Expr::PrimaryExpr(PrimaryExpr::Litteral(1)),
                            rparen: Token { 
                                kind: TokenKind::RParan,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 6,
                                    },
                                    size : 1,
                                    data : ")",
                                    source : &buf
                                }
                            },
                            then: Box::new(Statement::Block(Block {
                                lcurly: Token { 
                                    kind: TokenKind::LCurly,
                                    span: SourceSpan {
                                        start : SourceCursor {
                                            data_ref : "",
                                            line : 1,
                                            collumn : 8,
                                        },
                                        size : 1,
                                        data : "{",
                                        source : &buf
                                    }
                                },
                                body : Vec::new(),
                                rcurly: Token { 
                                    kind: TokenKind::RCurly,
                                    span: SourceSpan {
                                        start : SourceCursor {
                                            data_ref : "",
                                            line : 1,
                                            collumn : 9,
                                        },
                                        size : 1,
                                        data : "}",
                                        source : &buf
                                    }
                                }
                            })),
                            else_kw: None,
                            _else: None 
                        }),
                        Statement::If(If {
                            if_kw: Token { 
                                kind: TokenKind::Keyword(KeywordType::If),
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 11,
                                    },
                                    size : 2,
                                    data : "if",
                                    source : &buf
                                }
                            },
                            lparen: Token { 
                                kind: TokenKind::LParan,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 14,
                                    },
                                    size : 1,
                                    data : "(",
                                    source : &buf
                                }
                            },
                            cond: Expr::PrimaryExpr(PrimaryExpr::Litteral(0)),
                            rparen: Token { 
                                kind: TokenKind::RParan,
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 16,
                                    },
                                    size : 1,
                                    data : ")",
                                    source : &buf
                                }
                            },
                            then: Box::new(Statement::Block(Block {
                                lcurly: Token { 
                                    kind: TokenKind::LCurly,
                                    span: SourceSpan {
                                        start : SourceCursor {
                                            data_ref : "",
                                            line : 1,
                                            collumn : 18,
                                        },
                                        size : 1,
                                        data : "{",
                                        source : &buf
                                    }
                                },
                                body : vec![
                                    Statement::Return(Return {
                                        return_kw: Token { 
                                            kind: TokenKind::Keyword(KeywordType::Return),
                                            span: SourceSpan {
                                                start : SourceCursor {
                                                    data_ref : "",
                                                    line : 1,
                                                    collumn : 20,
                                                },
                                                size : 6,
                                                data : "return",
                                                source : &buf
                                            }
                                        },
                                        value: Expr::PrimaryExpr(PrimaryExpr::Litteral(1)),
                                        semicolon: Token { 
                                            kind: TokenKind::Semicolon,
                                            span: SourceSpan {
                                                start : SourceCursor {
                                                    data_ref : "",
                                                    line : 1,
                                                    collumn : 28,
                                                },
                                                size : 1,
                                                data : ";",
                                                source : &buf
                                            }
                                        }
                                    })
                                ],
                                rcurly: Token { 
                                    kind: TokenKind::RCurly,
                                    span: SourceSpan {
                                        start : SourceCursor {
                                            data_ref : "",
                                            line : 1,
                                            collumn : 30,
                                        },
                                        size : 1,
                                        data : "}",
                                        source : &buf
                                    }
                                }
                            })),
                            else_kw: Some(Token { 
                                kind: TokenKind::Keyword(KeywordType::Else),
                                span: SourceSpan {
                                    start : SourceCursor {
                                        data_ref : "",
                                        line : 1,
                                        collumn : 32,
                                    },
                                    size : 4,
                                    data : "else",
                                    source : &buf
                                }
                            }),
                            _else: Some(Box::new(Statement::Return(Return {
                               return_kw: Token { 
                                    kind: TokenKind::Keyword(KeywordType::Return),
                                    span: SourceSpan {
                                        start : SourceCursor {
                                            data_ref : "",
                                            line : 1,
                                            collumn : 37,
                                        },
                                        size : 6,
                                        data : "return",
                                        source : &buf
                                    }
                                },
                                value: Expr::PrimaryExpr(PrimaryExpr::Litteral(0)),
                                semicolon: Token { 
                                    kind: TokenKind::Semicolon,
                                    span: SourceSpan {
                                        start : SourceCursor {
                                            data_ref : "",
                                            line : 1,
                                            collumn : 45,
                                        },
                                        size : 1,
                                        data : ";",
                                        source : &buf
                                    }
                                }
                            })))
                        })
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }
    /*
    #[test]
    fn test_call() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("print(0);return pow(input(), 2);")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::Call(Call("print".to_string(), vec![
                            Expr::PrimaryExpr(PrimaryExpr::Litteral(0))
                        ])),
                        Statement::Return(
                            Expr::PrimaryExpr(PrimaryExpr::Call(Call(
                                "pow".to_string(),
                                vec![
                                    Expr::PrimaryExpr(PrimaryExpr::Call(Call(
                                            "input".to_string(),
                                            Vec::new()
                                    ))),
                                    Expr::PrimaryExpr(PrimaryExpr::Litteral(2))
                                ]
                            )))
                        )
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_var_set() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("n = 1;")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::VarSet("n".to_string(), Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_while_loop() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("while (1) {} loop print(0);")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::While(
                            Expr::PrimaryExpr(PrimaryExpr::Litteral(1)),
                            Box::new(Statement::Block(Vec::new()))
                        ),
                        Statement::Loop(
                            Box::new(Statement::Call(Call(
                                "print".to_string(),
                                vec![
                                    Expr::PrimaryExpr(PrimaryExpr::Litteral(0))    
                                ]
                            )))
                        )
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_unary_expr() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("print(-1);print(+2);")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::UnaryExpr(UnaryOp::Minus, PrimaryExpr::Litteral(1))
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::UnaryExpr(UnaryOp::Plus, PrimaryExpr::Litteral(2))
                            ]
                        ))
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_primary_expr() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("print(1);print(input());print(n);print((-1));")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::PrimaryExpr(PrimaryExpr::Litteral(1))
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::PrimaryExpr(PrimaryExpr::Call(Call(
                                    "input".to_string(),
                                    Vec::new()
                                )))
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::PrimaryExpr(PrimaryExpr::Ident("n".to_string()))
                            ]
                        )),
                        Statement::Call(Call(
                                "print".to_string(),
                                vec![
                                    Expr::PrimaryExpr(PrimaryExpr::Expr(Box::new(
                                        Expr::UnaryExpr(
                                            UnaryOp::Minus,
                                            PrimaryExpr::Litteral(1)
                                        )
                                    )))
                                ]
                        ))
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_binexpr(){
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from(r#"
                print(1+1);
                print(1-1);
                print(1*1);
                print(1/1);
                print(1==1);
                print(1!=1);
                print(1>1);
                print(1>=1);
                print(1<=1);
                print(1<1);
                print(1<<1);
                print(1>>1);
            "#)
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Add)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Sub)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Mul)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Div)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Equal)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::NotEqual)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Greater)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::GreaterEqual)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::LesserEqual)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::Lesser)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::LShift)
                            ]
                        )),
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    Box::new(Expr::PrimaryExpr(PrimaryExpr::Litteral(1))),
                                    BinOp::RShift)
                            ]
                        ))
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_binexpr_precedence() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("print(-1*5+5==(+2-5)*5/8-5);")
        };
        match AstBuilder::from(Lexer::new(&buf)).parse_program() {
            Ok(p) => {
                assert_eq!(p,
                    Program(vec![
                        Statement::Call(Call(
                            "print".to_string(),
                            vec![
                                Expr::BinExpr(
                                    Box::new(Expr::BinExpr(
                                        Box::new(
                                            Expr::BinExpr(
                                                Box::new(Expr::UnaryExpr(
                                                    UnaryOp::Minus,
                                                    PrimaryExpr::Litteral(1))),
                                                Box::new(
                                                    Expr::PrimaryExpr(PrimaryExpr::Litteral(5))
                                                ),
                                                BinOp::Mul)
                                        ),
                                        Box::new(Expr::PrimaryExpr(
                                            PrimaryExpr::Litteral(5)
                                        )),
                                        BinOp::Add)),
                                    Box::new(Expr::BinExpr(
                                        Box::new(
                                            Expr::BinExpr(
                                                Box::new(Expr::PrimaryExpr(PrimaryExpr::Expr(
                                                    Box::new(
                                                        Expr::BinExpr(
                                                            Box::new(Expr::UnaryExpr(
                                                                UnaryOp::Plus,
                                                                PrimaryExpr::Litteral(2)
                                                            )),
                                                            Box::new(
                                                                Expr::PrimaryExpr(
                                                                    PrimaryExpr::Litteral(5)
                                                                )
                                                            ),
                                                            BinOp::Sub
                                                        )
                                                    )
                                                ))),
                                                Box::new(
                                                    Expr::BinExpr(
                                                        Box::new(
                                                            Expr::PrimaryExpr(
                                                                PrimaryExpr::Litteral(5)
                                                            )
                                                        ),
                                                        Box::new(
                                                            Expr::PrimaryExpr(
                                                                PrimaryExpr::Litteral(8)
                                                            )
                                                        ),
                                                        BinOp::Div
                                                    )
                                                ),
                                                BinOp::Mul
                                            )
                                        ),
                                        Box::new(
                                            Expr::PrimaryExpr(
                                                PrimaryExpr::Litteral(5)
                                            )
                                        ),
                                        BinOp::Sub
                                    )),
                                    BinOp::Equal
                                )
                            ]
                        ))
                    ])
                );
            },
            Err(e) => panic!("{}", e)
        }
    }*/
}
