use super::{compiler_error::CompilerError, source_buffer::{SourceBuffer, SourcePos}, source_span::SourceSpan};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
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
    Keyword(KeywordType),
    Ident,
    Int
}

#[derive(Debug, PartialEq, Eq)]
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
    pub kind : TokenKind,
    pub span : SourceSpan<'a>
}

pub struct Lexer {
    pub source : Rc<RefCell<SourceBuffer>>,
    pub pos : SourcePos
}

impl Lexer {
    pub fn new(source : Rc<RefCell<SourceBuffer>>) -> Self {
        Lexer {
            source,
            pos : SourcePos {
                index : 0,
                line : 1,
                collumn : 1
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        // TODO optimize
        self.source.borrow_mut().buffer.chars().nth(self.pos.index)
    }

    fn next_char(&mut self) -> Option<char> {
        if self.peek_char() == None {
            return None
        }
        self.pos.index += 1;
        if let Some(c) = self.peek_char() {
            match c {
                // ignore cariage return
                '\r' => {
                    self.next_char()
                },
                '\n' => {
                    self.pos.line += 1;
                    self.pos.collumn = 1;
                    Some(c)
                },
                _ => {
                    self.pos.collumn += 1;
                    Some(c)
                }
            }
        }else {
            None
        }
    }

    pub fn tokens(&mut self) -> LexerTokens {
        LexerTokens {
            lexer : self
        }
    }
}

pub struct LexerTokens<'a> {
    lexer : &'a mut Lexer
}

impl<'a> LexerTokens<'a> {
    /// ignore whitespaces
    fn skip_whitespaces_and_commants(&mut self) -> Option<Result<char, CompilerError>>{
        let mut current_char = self.lexer.peek_char()?;
        while current_char.is_whitespace() || current_char == '/' {
            let pre_char = current_char;
            current_char = self.lexer.next_char()?;
            if pre_char == '/' {
                if current_char == '*'  {
                    let mut start_pos = self.lexer.pos;
                    start_pos.collumn -= 1;
                    let error = Some(Err(CompilerError::new(
                            super::compiler_error::CompilerErrorKind::BadToken,
                            format!("unterminated block comment").as_str(),
                            self.lexer.source.borrow().path.to_str().unwrap(),
                            self.lexer.source.borrow().get_line(start_pos.line).unwrap(), 
                            start_pos.line as u32,
                            start_pos.collumn as u32,
                            None)));
                    if let Some(mut current_char) = self.lexer.next_char(){
                        if let Some(mut next_char) = self.lexer.next_char(){
                            while current_char!='*' && next_char != '/'{
                                if self.lexer.next_char() == None {
                                    return error;
                                }
                                current_char = self.lexer.peek_char().unwrap();
                                if self.lexer.next_char() == None {
                                    return error;
                                }
                                next_char = self.lexer.peek_char().unwrap();
                            }
                        }else {
                            return error
                        }
                    }else {
                        return error; 
                    }
                    current_char = self.lexer.next_char()?;
                }
                if current_char == '/' {
                    current_char = self.lexer.next_char()?;
                    while current_char != '\n' {
                        current_char = self.lexer.next_char()?;
                    }
                    current_char = self.lexer.next_char()?;
                }
            }
        }
        return Some(Ok(current_char));
    }

    /// read a span from the source file from the sgtart to the and of something that isn't a whitespace or comment
    fn read_span(&mut self) -> Option<Result<SourceSpan<'a>, CompilerError>> {
        let _ = self.skip_whitespaces_and_commants()?;
        let start_pos = self.lexer.pos;
        loop {
            if let Some(mut c)=self.lexer.next_char(){
                if c.is_whitespace() {
                    break;
                } else if c == '/' {
                    let slash_pos = self.lexer.pos;
                    c = self.lexer.next_char()?;
                    if c == '*' || c == '/' {
                        self.lexer.pos = slash_pos;
                        break;
                    }else {
                        self.lexer.pos = slash_pos;
                    }
                }
            }else {
                break;
            }
        }
        // unsafe because the rust compile doesn't want to compile this otherwise
        Some(Ok(SourceSpan::at(unsafe { std::mem::transmute(&*self.lexer.source.borrow())}, start_pos, self.lexer.pos)))
    }
}

impl<'a> Iterator for LexerTokens<'a> {
    type Item = Result<Token<'a>, CompilerError>;
   
    fn next(&mut self) -> Option<Self::Item> {
        let current_span = match self.read_span()? {
            Ok(s) => s,
            Err(e) => return Some(Err(e))
        };

        // test for integer litteral
        if current_span.data.chars().next()?.is_ascii_digit() {
            let mut end = 1;
            let mut current_char = current_span.data.chars().nth(end);
            while current_char.is_some() && (current_char?.is_ascii_digit()) {
                end += 1;
                current_char = current_span.data.chars().nth(end);
            }
            let mut end_pos = current_span.start;
            end_pos.collumn += end;
            end_pos.index += end;
            self.lexer.pos = end_pos;
            let span = SourceSpan::at(current_span.source, current_span.start, end_pos);
            return Some(Ok(Token { kind : TokenKind::Int, span } ))
        }

        if (current_span.end.index - current_span.start.index) > 1 {
            // test for the two chars tokens
            let string = current_span.data.get(..2).unwrap();
            if let Some(k) = match string {
                "==" => Some(TokenKind::DoubleEqual),
                "!=" => Some(TokenKind::NotEqual),
                ">=" => Some(TokenKind::GreaterEqual),
                "<=" => Some(TokenKind::LesserEqual),
                "<<" => Some(TokenKind::LShift),
                ">>" => Some(TokenKind::RShift),
                _ => None
            }
            {
                let mut end_pos = current_span.start;
                end_pos.collumn += 2;
                end_pos.index += 2;
                self.lexer.pos = end_pos;
                return Some(Ok(Token{
                    kind : k,
                    span : SourceSpan::at(current_span.source, current_span.start, end_pos)
                }))
            }
        }

        // test for single char tokens
        if let Some(k) = match current_span.data.chars().next()? {
                '{' => Some(TokenKind::LCurly),
                '}' => Some(TokenKind::RCurly),
                '(' => Some(TokenKind::LParan),
                ')' => Some(TokenKind::RParan),
                ';' => Some(TokenKind::Semicolon),
                '+' => Some(TokenKind::Plus),
                '*' => Some(TokenKind::Times),
                '/' => Some(TokenKind::Divider),
                '-' => Some(TokenKind::Minus),
                ',' => Some(TokenKind::Comma),
                '=' => Some(TokenKind::Equal),
                '>' => Some(TokenKind::Greater),
                '<' => Some(TokenKind::Lesser),
                _ => None
            }
        {
            let mut end_pos = current_span.start;
            end_pos.collumn += 1;
            end_pos.index += 1;
            self.lexer.pos = end_pos;
            return Some(Ok(Token{
                kind : k,
                span : SourceSpan::at(current_span.source, current_span.start, end_pos)
            }))
        }

        // test for ident
        if current_span.data.chars().next()?.is_alphabetic() || current_span.data.chars().next()? == '_' {
            let mut end = 1;
            let mut current_char = current_span.data.chars().nth(end);
            while current_char.is_some() && (current_char?.is_alphanumeric() || current_char? == '_') {
                end += 1;
                current_char = current_span.data.chars().nth(end);
            }
            let mut end_pos = current_span.start;
            end_pos.collumn += end;
            end_pos.index += end;
            self.lexer.pos = end_pos;
            let span = SourceSpan::at(current_span.source, current_span.start, end_pos);
            let kind =  match span.data {
                "if" => TokenKind::Keyword(KeywordType::If),
                "else" => TokenKind::Keyword(KeywordType::Else),
                "while" => TokenKind::Keyword(KeywordType::While),
                "loop" => TokenKind::Keyword(KeywordType::Loop),
                "return" => TokenKind::Keyword(KeywordType::Return),
                "break" => TokenKind::Keyword(KeywordType::Break),
                "continue" => TokenKind::Keyword(KeywordType::Continue),
                "var" => TokenKind::Keyword(KeywordType::Var),
                _ => TokenKind::Ident
            };
            return Some(Ok(Token { kind, span } ))
        }
        
        return Some(Err(CompilerError::new(
                    super::compiler_error::CompilerErrorKind::BadToken,
                    format!("bad token : {}", current_span.data).as_str(),
                    self.lexer.source.borrow().path.to_str().unwrap(),
                    self.lexer.source.borrow().get_line(current_span.start.line).unwrap(), 
                    current_span.start.line as u32,
                    current_span.start.collumn as u32,
                    None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_single_char() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("{}();+*/-,=><")
        };
        let tokens = vec![
            TokenKind::LCurly, 
            TokenKind::RCurly, 
            TokenKind::LParan, 
            TokenKind::RParan, 
            TokenKind::Semicolon, 
            TokenKind::Plus, 
            TokenKind::Times, 
            TokenKind::Divider, 
            TokenKind::Minus, 
            TokenKind::Comma, 
            TokenKind::Equal, 
            TokenKind::Greater, 
            TokenKind::Lesser
        ];
        let _ = Lexer::new(Rc::new(RefCell::new(buf))).tokens()
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .zip(tokens.iter())
            .map(|(x,y)| assert_eq!(x, *y));
    }

    #[test]
    fn test_keywords() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("if else while loop return break continue var ")
        };
        let keyword = vec![
            TokenKind::Keyword(KeywordType::If),
            TokenKind::Keyword(KeywordType::Else),
            TokenKind::Keyword(KeywordType::While),
            TokenKind::Keyword(KeywordType::Loop),
            TokenKind::Keyword(KeywordType::Return),
            TokenKind::Keyword(KeywordType::Break),
            TokenKind::Keyword(KeywordType::Continue),
            TokenKind::Keyword(KeywordType::Var)
        ];
        let _ = Lexer::new(Rc::new(RefCell::new(buf))).tokens()
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .zip(keyword.iter())
            .map(|(x,y)| assert_eq!(x, *y));
    }

    #[test]
    fn test_ident() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("_ll dqd /* sss */ ll6 ll_k_5 ssqdq 5ll 'l' lè ù")
        };

        let tokens = vec![
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
            None,
            None,
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
        ];

        let _ = Lexer::new(Rc::new(RefCell::new(buf))).tokens()
            .map(|x| x.ok().map(|x| x.kind))
            .zip(tokens.iter())
            .map(|(x,y)| assert_eq!(x, *y));
    }

    #[test]
    fn test_two_char() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("== != >= <= << >> ")
        };
        let tokens = vec![         
            TokenKind::DoubleEqual,
            TokenKind::NotEqual,
            TokenKind::GreaterEqual,
            TokenKind::LesserEqual,
            TokenKind::LShift,
            TokenKind::RShift,
        ];
        let _ = Lexer::new(Rc::new(RefCell::new(buf))).tokens()
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .zip(tokens.iter())
            .map(|(x,y)| assert_eq!(x, *y));
    }
}
