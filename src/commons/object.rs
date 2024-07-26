use crate::compiler::text_data::TextData;
use super::instructions::Instructions;

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
}
