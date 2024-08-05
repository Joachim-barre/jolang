use super::{compiler_error::CompilerError, source_buffer::{SourceBuffer, SourcePos}, source_span::SourceSpan};

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
    pub kind : TokenKind,
    pub span : SourceSpan<'a>
}

pub struct Lexer {
    pub source : SourceBuffer,
    pub pos : SourcePos
}

impl Lexer {
    pub fn new(source : SourceBuffer) -> Self {
        Lexer {
            source,
            pos : SourcePos {
                index : 0,
                line : 0,
                collumn : 0
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        // TODO optimize
        self.source.buffer.chars().nth(self.pos.index)
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
                    self.pos.collumn = 0;
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
    fn skip_whitespaces_and_commants(&mut self) -> Option<char>{
        let mut current_char = self.lexer.peek_char()?;
        while current_char.is_whitespace() || current_char == '/' {
            let pre_char = current_char;
            current_char = self.lexer.next_char()?;
            if pre_char == '/' {
                if current_char == '*'  {
                    current_char = self.lexer.next_char()?;   
                    while current_char!='*' && self.lexer.next_char()? != '/'{
                        current_char = self.lexer.next_char()?;
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
        return Some(current_char);
    }
}

impl<'a> Iterator for LexerTokens<'a> {
    type Item = Result<Token<'a>, CompilerError>;
   
    fn next(&mut self) -> Option<Self::Item> {
        let mut current_char = self.ignore_whitespaces()?;
        let maybe_next_char = self.lexer.next_char();
        // ignore comments
        if current_char == '/' {
            if let Some(next_char) = maybe_next_char {
                if next_char == '/' {
                    while self.lexer.next_char().map_or(false, |x| x!='\n') {}
                    current_char = self.lexer.next_char()?;
                    maybe_next_char = self.lexer.next_char();
                }else if next_char == '*' {
                    let start_pos = self.lexer.pos;
                    start_pos.collumn -= 1;
                    if let Some(c) = self.lexer.next_char(){
                        current_char = c;
                    }else {
                        return Some(Err(CompilerError::new(
                            super::compiler_error::CompilerErrorKind::BadToken,
                            format!("unterminated block comment").as_str(),
                            self.lexer.source.path.to_str().unwrap(),
                            self.lexer.source.get_line(start_pos.line).unwrap(), 
                            start_pos.line as u32,
                            start_pos.collumn as u32,
                            None)))
                    }
                    if let Some(c) = self.lexer.next_char(){
                        next_char = c;
                    }else {
                        return Some(Err(CompilerError::new(
                            super::compiler_error::CompilerErrorKind::BadToken,
                            format!("unterminated block comment").as_str(),
                            self.lexer.source.path.to_str().unwrap(),
                            self.lexer.source.get_line(start_pos.line).unwrap(), 
                            start_pos.line as u32,
                            start_pos.collumn as u32,
                            None)))
                    }
                    while current_char != '*' && next_char != '/' {
                        if let Some(c) = self.lexer.next_char(){
                            next_char = c;
                        }else {
                            return Some(Err(CompilerError::new(
                                super::compiler_error::CompilerErrorKind::BadToken,
                                format!("unterminated block comment").as_str(),
                                self.lexer.source.path.to_str().unwrap(),
                                self.lexer.source.get_line(start_pos.line).unwrap(), 
                                start_pos.line as u32,
                                start_pos.collumn as u32,
                                None)))
                        }
                    }
                    current_char = self.lexer.next_char()?;
                    maybe_next_char = self.lexer.next_char();
                }
            }
        }
        let current_pos = self.lexer.pos;
        // test for single char tokens
        if let Some(k) = match current_char {
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
                _ => None
            }
        {
            return Some(Ok(Token{
                kind : k,
                span : SourceSpan::at(&self.lexer.source, current_pos, self.lexer.pos)
            }))
        }
    }
}
