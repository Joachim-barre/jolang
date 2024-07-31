use std::fs::File;
use std::io::{BufReader, Lines, BufRead};
use std::option::Option;
use anyhow::{anyhow, Result};

#[derive(Debug)]
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

    pub fn find_headers(&mut self) -> Result<()>{
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
            return Err(anyhow!("to many headers"))
        }

        for header in headers {
            match header.1.as_str() {
                ".DATA" => {
                    if self.data_start != None {
                        return Err(anyhow!("header found twice : .DATA"))
                    }
                    self.data_start = Some(header.0 as u64);
                    if self.text_start != None {
                        self.text_end = self.data_start;
                    }
                },
                ".TEXT" => {
                    if self.text_start != None {
                        return Err(anyhow!("header found twice : .TEXT"))
                    }
                    self.text_start = Some(header.0 as u64);
                    if self.data_start != None {
                        self.data_end = self.text_start;
                    }
                },
                _ => {
                    return Err(anyhow!("bad header"))
                }
            }
        }
        Ok(())
    }

    pub fn parse_data(&self) -> Result<Vec<i64>> {
        if self.data_start == None {
            return Err(anyhow!("you need to parse headers first"))
        }
        let mut data_size : String = String::new();
        let mut data_default : String = String::new();
        let mut numbers : Vec<i64> = self.lines()
            .enumerate()
            .filter_map(|(line, x)| x.ok().map(|y| (line, String::from(y))))
            .filter(|(_line, x)| !x.starts_with("#"))
            .enumerate()
            .filter_map(|(n, x)| { if n==0 { data_size=x.1; return None} else if n==1 { data_default=x.1; return None} else { return Some(x) } }  )
            .filter(|(line, _x)| (line) > &(self.data_start.unwrap() as usize))
            .filter_map(|(line,x)| if (self.data_end == None) || (line<(self.data_end.unwrap() as usize)) { Some(x) } else { None })
            .map(|x| x.parse::<i64>().unwrap())
            .collect();      

        let data_size = data_size.parse::<usize>().unwrap();
        
        if numbers.len() > data_size {
            return Err(anyhow!("to many numbers in data section"))
        }
        
        let data_default = data_default.parse::<i64>().unwrap();

        while numbers.len() < data_size  { numbers.push(data_default) }

        return Ok(numbers)
    }
}
