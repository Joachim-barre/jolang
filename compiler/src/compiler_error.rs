use std::error::Error;
use std::fmt;

macro_rules! enum_str {
    (
     $(#[$meta:meta])*
    enum $name:ident {
        $($(#[$meta2:meta])? $variant:ident $(= $val:expr)?),* $(,)?
    }) => {
        $(#[$meta])*
        pub enum $name {
            $( $(#[$meta2])* $variant $(= $val)?),*
        }

        impl $name {
            fn to_str(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

enum_str!{
    #[derive(Debug, PartialEq, Clone)]
    enum CompilerErrorKind {
        /// Unknown token
        BadToken,
        UnexpectedToken,
        Expected,
        UnderlaredVariable,
        ReeclaretedVariable,
        UnknownFunction,
    }
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    kind : CompilerErrorKind,
    message : String,
    file : String,
    line : String,
    line_number : u32,
    col_number : u32,
    /// message, snippet
    hint : Option<(String, String)>
}

impl CompilerError {
    pub fn new(kind: CompilerErrorKind, message: &str, file: &str, line : &str, line_number: u32, col_number: u32, hint_ : Option<(&str, &str)>) -> Self {
        let hint;
        if let Some(hint_) = hint_ {
            hint = Some((hint_.0.to_string(), hint_.1.to_string()))
        }else {
            hint = None
        }

        Self {
            kind,
            message: message.to_string(),
            file: file.to_string(),
            line: line.to_string(),
            line_number,
            col_number : if col_number == 0 { 1 } else { col_number }, 
            hint
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = format!(
            "\x1b[91;1mError\x1b[0;1m[{}]: {}\x1b[0;0m\n  \x1b[36;1m-->\x1b[0m {}:{}:{}\n    \x1b[36;1m|\n{:4}|\x1b[0m {}\n    \x1b[36;1m| \x1b[91m{}\n\x1b[0m",
            self.kind.to_str(),
            self.message,
            self.file,
            self.line_number,
            self.col_number,
            self.line_number,
            self.line,
            " ".repeat((self.col_number - 1) as usize) + "^~~~"
        );

        if let Some(hint) = &self.hint {
            formatted.push_str(&format!(
                "\x1b[92;1mhelp\x1b[0m: {}\n    \x1b[36;1m|\n{:4}| {}\n    |\x1b[0m\n",
                hint.0,
                self.line_number,
                hint.1
            ));
        }

        write!(f, "{}", formatted)
    }
}

impl Error for CompilerError {}
