use crate::{cli::run::RunArgs, commons::object::Object};
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
    dbg!(Object::load(&mut file)?);
    todo!();
}
