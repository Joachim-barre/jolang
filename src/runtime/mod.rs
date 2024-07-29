use crate::{cli::run::RunArgs, commons::{instructions::Instructions, object::Object}};
use std::{result::Result, fs::{File, OpenOptions}};

pub fn run(args : RunArgs) -> Result<(), String> {
    if !args.file.is_local() {
        return Err(String::from("please input a local file"))
    }
    let mut file : File;
    match OpenOptions::new().read(true).write(false).truncate(false).append(false).open(args.file.clone().as_os_str()) {
        Ok(f) => {
            file = f
        }
        Err(_) => {
            return Err(String::from("can't open file"))
        }
    }
    println!("loading object {}", args.file);
    let object = Object::load(&mut file)?;
    
    let mut blocks : Vec<Vec<Instructions>> = Vec::new();

    let mut block_offset : usize = 0; 

    let mut current_block : Vec<Instructions> = object.text;

    for jump in &object.jumps[1..] {
        let next_block = current_block.split_off(*jump as usize - block_offset);
        block_offset += *jump as usize;
        blocks.push(current_block);
        current_block = next_block;
    }
    blocks.push(current_block);

    todo!();
}
