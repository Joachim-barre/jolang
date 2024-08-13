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
    pub fn new(source : &'a SourceBuffer) -> Self{
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
                }else if current_char == '/' {
                    current_char = self.reader.next_char()?;
                    while current_char != '\n' {
                        current_char = self.reader.next_char()?;
                    }
                    current_char = self.reader.next_char()?;
                }
            }else {
                current_char = self.reader.next_char()?
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
            let start = self.reader.current_cursor;
            self.reader.next_char();
            let mut size = 1;
            while self.reader.peek_char().is_some() && (self.reader.peek_char().unwrap().is_ascii_digit()) {
                self.reader.next_char();
                size += 1;
            }
            let span : SourceSpan<'a> = unsafe {std::mem::transmute(SourceSpan::at(self.reader.source, start, size)) };
            return Some(Ok(Token { kind : TokenKind::Int, span } ))
        }

        if self.reader.get_cursor().data_ref.chars().nth(1).is_some() {
            // test for the two chars tokens
            let string = self.reader.get_cursor().data_ref.chars().take(2).collect::<String>();
            if let Some(k) = match string.as_str() {
                "==" => Some(TokenKind::DoubleEqual),
                "!=" => Some(TokenKind::NotEqual),
                ">=" => Some(TokenKind::GreaterEqual),
                "<=" => Some(TokenKind::LesserEqual),
                "<<" => Some(TokenKind::LShift),
                ">>" => Some(TokenKind::RShift),
                _ => None
            }
            {
            self.reader.next_char();
            self.reader.next_char();
                return Some(Ok(Token{
                    kind : k,
                    span : unsafe { std::mem::transmute(SourceSpan::at(self.reader.source, self.reader.get_cursor().clone(), 2)) }
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
            self.reader.next_char();
            return Some(Ok(Token{
                kind : k,
                span : unsafe { std::mem::transmute(SourceSpan::at(self.reader.source, self.reader.get_cursor().clone(), 1)) }
            }))
        }

        // test for ident
        if self.reader.peek_char()?.is_alphabetic() || self.reader.peek_char()? == '_' {
            let start = self.reader.current_cursor.clone();
            let mut size = 1;
            while self.reader.peek_char().is_some() && (self.reader.peek_char()?.is_alphanumeric() || self.reader.peek_char()? == '_') {
                self.reader.next_char();
                size += 1;
            }
            let span : SourceSpan<'a> = unsafe { std::mem::transmute(SourceSpan::at(self.reader.source, start, size-1)) };
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
                    format!("bad token : {}", self.reader.peek_char()?).as_str(),
                    self.reader.source.path.to_str().unwrap(),
                    self.reader.source.get_line(self.reader.get_cursor().line).unwrap(), 
                    self.reader.get_cursor().line as u32,
                    self.reader.get_cursor().collumn as u32,
                    None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
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
        let tokens2 : Vec<_> = Lexer::new(&buf)
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .collect();
        assert_eq!(tokens2, tokens);
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
        let tokens2 : Vec<_> = Lexer::new(&buf)
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .collect();
        assert_eq!(tokens2, keyword);
    }

    #[test]
    fn test_ident() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("_ll dqd /* sss */ ll6 ll_k_5 ssqdq 5ll 'l' lè ù")
        };

        let tokens : Vec<_> = vec![
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
            None,
            None,
            Some(TokenKind::Ident),
            Some(TokenKind::Ident),
        ].iter().map(|x| (x.is_none(), x.clone().unwrap_or(TokenKind::Int)))
            .collect();

        let tokens2 : Vec<_> = Lexer::new(&buf)
            .map(|x| x.ok())
            .map(|x| (x.is_none(), x.map_or(TokenKind::Int, |x| x.kind)))
            .collect();
        assert_eq!(tokens2, tokens);
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
        let tokens2 : Vec<_> = Lexer::new(&buf)
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .collect();
        assert_eq!(tokens2, tokens);
    }

    #[test]
    fn test_unicode_ident() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("я_не_говорю_по-русски 😄 私も彼らも日本語を話せません")
        };
        let tokens : Vec<_> = vec![
            Some(TokenKind::Ident),
            None,
            Some(TokenKind::Ident)
        ].iter().map(|x| (x.is_none(), x.clone().unwrap_or(TokenKind::Int)))
            .collect();

        let tokens2 : Vec<_> = Lexer::new(&buf)
            .map(|x| x.ok())
            .map(|x| (x.is_none(), x.map_or(TokenKind::Int, |x| x.kind)))
            .collect();
        assert_eq!(tokens2, tokens);
    }

    #[test]
    fn test_mixed_tokens() {
        let buf = SourceBuffer {
            path : PathBuf::from("test.jol"),
            buffer : String::from("var _n = 0;// test commant\n _n = _n + 1; if (_n != 1) {return 0}\n /* a block comment\n// this is a comment in a comment and should do nothing\n*/ var test = 0; while (1) { test = input();\n print(test); } ")
        };
        let tokens = vec![         
            TokenKind::Keyword(KeywordType::Var),
            TokenKind::Ident,
            TokenKind::Equal,
            TokenKind::Int,
            TokenKind::Semicolon,
            TokenKind::Ident,
            TokenKind::Equal,
            TokenKind::Ident,
            TokenKind::Plus,
            TokenKind::Int,
            TokenKind::Semicolon,
            TokenKind::Keyword(KeywordType::If),
            TokenKind::LParan,
            TokenKind::Ident,
            TokenKind::NotEqual,
            TokenKind::Int,
            TokenKind::RParan,
            TokenKind::LCurly,
            TokenKind::Keyword(KeywordType::Return),
            TokenKind::Int,
            TokenKind::RCurly,
            TokenKind::Keyword(KeywordType::Var),
            TokenKind::Ident,
            TokenKind::Equal,
            TokenKind::Int,
            TokenKind::Semicolon,
            TokenKind::Keyword(KeywordType::While),
            TokenKind::LParan,
            TokenKind::Int,
            TokenKind::RParan,
            TokenKind::LCurly,
            TokenKind::Ident,
            TokenKind::Equal,
            TokenKind::Ident,
            TokenKind::LParan,
            TokenKind::RParan,
            TokenKind::Semicolon,
            TokenKind::Ident,
            TokenKind::LParan,
            TokenKind::Ident,
            TokenKind::RParan,
            TokenKind::Semicolon,
            TokenKind::RCurly
        ];
        let tokens2 : Vec<_> = Lexer::new(&buf)
            .map(|x| { assert!(x.is_ok()); x.ok().map(|x| x.kind).unwrap()})
            .collect();
        assert_eq!(tokens2, tokens);
    }
}
