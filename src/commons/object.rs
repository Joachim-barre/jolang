use super::instructions::Instruction;
use std::{fs::File, io::{Read, Write}};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Object {
    pub extfn : Vec<(String, u8, bool)>
}

impl Object {
    
}
