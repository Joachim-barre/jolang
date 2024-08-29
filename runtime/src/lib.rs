use std::{fs::OpenOptions, path::PathBuf};
use anyhow::Result;
use jolang_shared::ir::{reader::read, IrObject};
mod llvm;

pub trait Runtime {
    fn new() -> Self
        where Self : Sized;
    fn load(&mut self, object : IrObject) -> Result<()>;
    fn run(&mut self) -> i64;
}

pub fn run(file : PathBuf) -> Result<i64> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file)?;
    dbg!(read(&mut file)?);
    todo!()
}
