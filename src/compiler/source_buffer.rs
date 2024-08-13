use std::path::PathBuf;
use std::fs::read_to_string;
use anyhow::Error;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct SourcePos {
    pub index : usize,
    pub line : usize,
    pub collumn : usize,
}

pub struct SourceBuffer {
    pub path : PathBuf,
    pub buffer : String
}

impl std::fmt::Debug for SourceBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

impl SourceBuffer {
    pub fn open(path : PathBuf) -> Result<Self, Error>{
        match read_to_string(path.clone()) {
            Ok(s) =>  Ok(SourceBuffer {
                path,
                buffer : s
            }),
            Err(e) => Err(e.into())
        }
    }

    pub fn get_line(&self, line : usize) -> Option<&str> {
        self.buffer.lines().nth(line-1)
    }
}
