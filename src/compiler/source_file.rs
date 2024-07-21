use std::fs::File;
use std::io::{BufReader, Lines, BufRead};
use std::option::Option;

pub struct SourceFile {
    pub file: File,
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
            file,
            text_start : None,
            text_end : None,
            data_start : None,
            data_end : None
        }
    }
}

impl SourceFile {
    pub fn lines<'a>(&'a self) -> Lines<BufReader<&'a File>> {
        return BufReader::new(&self.file).lines()
    }

}
