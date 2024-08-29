use std::{fs::OpenOptions, path::PathBuf};
use anyhow::Result;
use jolang_shared::ir::{reader::read, IrObject};
mod llvm;

pub trait Runtime {
    fn load(&mut self, object : IrObject);

    fn run(&mut self) -> i64;
}

pub fn run(file : PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file)?;
    dbg!(read(&mut file)?);
    todo!()
}
