use std::fs::File;
use std::io::{BufReader, Lines, BufRead};
use std::option::Option;
use std::result::Result;

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

    pub fn find_headers(&mut self) -> Result<(), String>{
        self.text_start = None;
        self.text_end = None;
        self.data_start = None;
        self.data_end = None;
        let headers: Vec<(usize, String)> = self.lines()
            .enumerate()
            .filter_map(|(idx, line)| line.ok().map(|line| (idx, line)))
            .filter(|(_, line)| line.starts_with('.'))
            .collect();

        if headers.len() != 2{
            return Err("to many headers".to_string())
        }

        for header in headers {
            match header.1.as_str() {
                ".DATA" => {
                    if self.data_start != None {
                        return Err("header found twice : .DATA".to_string())
                    }
                    self.data_start = Some(header.0 as u64);
                    if self.text_start != None {
                        self.text_end = self.data_start;
                    }
                },
                ".TEXT" => {
                    if self.text_start != None {
                        return Err("header found twice : .TEXT".to_string())
                    }
                    self.text_start = Some(header.0 as u64);
                    if self.data_start != None {
                        self.data_end = self.data_start;
                    }
                },
                _ => {
                    return Err("bad header".to_string())
                }
            }
        }
        Ok(())
    }
}
