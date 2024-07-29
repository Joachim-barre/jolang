use crate::{cli::run::RunArgs, commons::{instructions::{self, Instructions}, object::Object}};
use std::{result::Result, fs::{File, OpenOptions}};
use inkwell::{context::Context, AddressSpace};

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

    let data_size = object.data.len();

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i64_type = context.i64_type();
    let ptr_type = i64_type.ptr_type(AddressSpace::default());
    let main_fn_type = i64_type.fn_type(&[i64_type.into(), ptr_type.into()], false);
    let main_fn = module.add_function("main", main_fn_type, None);
    
    let entry_block = context.append_basic_block(main_fn, "entry");
    
    let blocks : Vec<_> = blocks.iter()
        .enumerate()
        .map(|(i, x)| (format!("block{}", i.to_string()), x))
        .map(|(i, x)| (context.append_basic_block(main_fn, &i), x))
        .collect();

    todo!();
}
