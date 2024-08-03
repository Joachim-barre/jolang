use super::{source_buffer::{SourceBuffer, SourcePos}, source_span::SourceSpan};

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

impl<'a> Iterator for LexerTokens<'a> {
    type Item = Result<Token<'a>, CompilerError>;
   
    fn next(&mut self) -> Option<Self::Item> {
        let mut current_char = self.lexer.peek_char()?;
        // ignore whitespace
        while current_char.is_whitespace() {
            current_char = self.lexer.next_char()?;
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
            self.lexer.next_char();
            return Some(Ok(Token{
                kind : k,
                span : SourceSpan::at(&self.lexer.source, current_pos, self.lexer.pos)
            }))
        }
    }
}
