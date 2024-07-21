use std::fs::File;
use std::option::Option;

pub struct SourceFile {
    pub file : File,
    pub text_start : Option<u64>,
    pub text_end : Option<u64>,
    pub data_start : Option<u64>,
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
