use super::{compiler_error::CompilerError, source_buffer::SourceBuffer, source_reader::SourceReader, source_span::SourceSpan};
use std::{cell::RefCell, rc::Rc};

macro_rules! enum_str {
    (
     $(#[$meta:meta])*
    pub enum $name:ident {
        $($(#[$meta2:meta])? $variant:ident $(($data:ident))? $(= $val:expr)?),* $(,)?
    }) => {
        $(#[$meta])*
        pub enum $name {
            $( $(#[$meta2])* $variant $(($data))? $(= $val)?),*
        }

        #[allow(unused, non_snake_case)]
        impl $name {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $($name::$variant $(($data))? => stringify!($variant)),*
                }
            }
        }
    };
}
enum_str!(
#[derive(Debug, PartialEq, Eq, Clone)]
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
});

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind : TokenKind,
    pub span : SourceSpan<'a>
}

pub struct Lexer<'a> {
    pub reader : SourceReader<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(source : &SourceBuffer) -> Self{
        Self {
            reader : SourceReader::from(source)
        }
    }

    /// ignore whitespaces
    fn skip_whitespaces_and_commants(&mut self) -> Option<Result<char, CompilerError>>{
        let mut current_char = self.reader.peek_char()?;
        while current_char.is_whitespace() || current_char == '/' {
            if current_char == '/' {
                if self.reader.get_cursor().data_ref.chars().nth(1)? == '*'  {
                    let error = Some(Err(CompilerError::new(
                            super::compiler_error::CompilerErrorKind::BadToken,
                            format!("unterminated block comment").as_str(),
                            self.reader.source.path.to_str().unwrap(),
                            self.reader.source.get_line(self.reader.get_cursor().line).unwrap(), 
                            self.reader.get_cursor().line as u32,
                            self.reader.get_cursor().collumn as u32,
                            None)));
                    let _ = self.reader.next_char();
                    if let Some(mut current_char) = self.reader.next_char(){
                        if let Some(mut next_char) = self.reader.next_char(){
                            while current_char!='*' && next_char != '/'{
                                if self.reader.next_char() == None {
                                    return error;
                                }
                                current_char = next_char;
                                next_char = self.reader.peek_char().unwrap();
                            }
                        }else {
                            return error
                        }
                    }else {
                        return error; 
                    }
                    current_char = self.reader.next_char()?;
                    continue;
                }
                if current_char == '/' {
                    current_char = self.reader.next_char()?;
                    while current_char != '\n' {
                        current_char = self.reader.next_char()?;
                    }
                    current_char = self.reader.next_char()?;
                }
            }
        }
        return Some(Ok(current_char));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, CompilerError>;
   
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces_and_commants()?;
        // test for integer litteral
        if self.reader.peek_char().unwrap().is_ascii_digit() {
            let start = self.reader.get_cursor().clone();
            self.reader.next_char();
            let mut size = 1;
            self.reader.next_char();
            while self.reader.peek_char().is_some() && (self.reader.peek_char().unwrap().is_ascii_digit()) {
                self.reader.next_char();
                size += 1;
            }
            let span = SourceSpan::at(self.reader.source, start, size);
            return Some(Ok(Token { kind : TokenKind::Int, span } ))
        }

        if self.reader.get_cursor().data_ref.chars().nth(1).is_some() {
            // test for the two chars tokens
            let string = self.reader.get_cursor().data_ref.chars().take(2).collect::<String>().as_str();
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
                return Some(Ok(Token{
                    kind : k,
                    span : SourceSpan::at(self.reader.source, self.reader.get_cursor().clone(), 2)
                }))
            }
        }

        // test for single char tokens
        if let Some(k) = match self.reader.peek_char()? {
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
            return Some(Ok(Token{
                kind : k,
                span : SourceSpan::at(self.reader.source, self.reader.get_cursor().clone(), 1)
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
