use std::fs::File;
use std::io::{BufReader, Lines};
use std::option::Option;

pub struct SourceFile {
    pub reader : BufReader<File>,
    // first text line
    pub text_start : Option<u64>,
    // last text line
    pub text_end : Option<u64>,
    // first data line
    pub data_start : Option<u64>,
    // last data line
    pub data_end : Option<u64>
}

impl From<File> for SourceFile {
    fn from(file: File) -> Self {
        SourceFile {
            reader : BufReader::new(file),
            text_start : None,
            text_end : None,
            data_start : None,
            data_end : None
        }
    }
}
