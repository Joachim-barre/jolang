use crate::compiler::text_data::TextData;
use super::instructions::Instructions;
use std::{fs::File, io::{Read, Write}};

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

    pub fn load(file : &mut File) -> Result<Self, String> {
        let mut bytes :  Vec<u8> = Vec::new();

        match file.read_to_end(&mut bytes) {
            Ok(_) => {},
            Err(_) => {
                return Err("can't read file".to_string());
            }
        };

        if bytes.len() < 36 {
            return Err("object file too small".to_string());
        }

        if bytes[..4] != "\0JOO".as_bytes()[..4] {
            return Err("bad header (unknown file format)".to_string())
        }

        let main_jump = u64::from_le_bytes(bytes[4..12].try_into().unwrap());
        let jump_len = u64::from_le_bytes(bytes[12..20].try_into().unwrap()); 
        if bytes.len() < (36 + jump_len*8) as usize {
            return Err("bad jump table size".to_string())
        }
        let mut jumps = bytes.split_off(20);
        bytes = jumps.split_off((jump_len*8) as usize);
        let jumps : Vec<u64> = jumps.chunks_exact(8)
            .map(|x| u64::from_le_bytes(x.try_into().unwrap()))
            .collect();
        let data_len = u64::from_le_bytes(bytes[..8].try_into().unwrap());
        if bytes.len() < (16 + data_len*8) as usize {
            return Err("bad data section size".to_string())
        } 
        let mut data = bytes.split_off(8);
        bytes = data.split_off((data_len*8) as usize);
        let data : Vec<i64> = data.chunks_exact(8)
            .map(|x| i64::from_le_bytes(x.try_into().unwrap()))
            .collect();
        let text_len = u64::from_le_bytes(bytes[..8].try_into().unwrap());
        if bytes.len() != (8+text_len) as usize {
            return Err(String::from("bad text section size"))
        }
        let text : Vec<Instructions> = bytes.iter()
            .skip(8)
            .map(|x| Instructions::from(*x))
            .collect();

        let object = Self {
            main_jump,
            jumps,
            data,
            text
        };
        
        Ok(object)
    }
}
