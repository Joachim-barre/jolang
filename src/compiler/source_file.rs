use std::fs::File;

pub struct SourceFile {
    pub file : File
}

impl From<File> for SourceFile {
    fn from(file: File) -> Self {
        SourceFile {
            file
        }
    }
}
