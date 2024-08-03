use std::path::PathBuf;
use std::fs::read_to_string;
use anyhow::Error;

pub struct SourceBuffer {
    pub path : PathBuf,
    pub buffer : String
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
}
