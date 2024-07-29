use crate::compiler::text_data::TextData;
use super::instructions::Instructions;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct Object {
    pub main_jump : u64,
    pub jumps : Vec<u64>,
    pub data : Vec<i64>,
    pub text : Vec<Instructions>
}

impl Object {
    pub fn build(text : TextData, data : Vec<i64>) -> Result<Self, String> {
        if data.len() < 1 || data[0] < 0 || data[0] >= text.jumps.len().try_into().unwrap() {
            return Err(String::from("bad entry point"))
        }
        Ok(Object {
            main_jump : data[0] as u64,
            jumps : text.jumps.iter()
                .map(|x| (*x as u64))
                .collect(),
            data,
            text : text.instructions
        })
    }

    pub fn save(&self, file : &mut File) -> Result<(), String> {
        let mut bytes : Vec<u8> = Vec::new();
        
        bytes.extend_from_slice("\0JOO".as_bytes());
        bytes.extend_from_slice(&self.main_jump.to_le_bytes());
        bytes.extend_from_slice(&(self.jumps.len() as u64).to_le_bytes());
        bytes.extend(self.jumps.iter().flat_map(|x| x.to_le_bytes()));
        bytes.extend_from_slice(&(self.data.len() as u64).to_le_bytes());
        bytes.extend(self.data.iter().flat_map(|x| x.to_le_bytes()));
        bytes.extend_from_slice(&(self.text.len() as u64).to_le_bytes());
        bytes.extend(self.text.iter().flat_map(|x| Into::<u8>::into(x.clone()).to_le_bytes()));

        if !file.write(&bytes[..]).ok().map_or(false, |x| x==bytes.len()) {
            return Err(String::from("failed to write the compiled object"));
        }
        Ok(())
    }
}
