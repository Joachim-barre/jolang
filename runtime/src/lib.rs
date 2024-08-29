use std::{fs::OpenOptions, path::PathBuf};
use anyhow::Result;
use jolang_shared::ir::{reader::read, IrObject};
use llvm::LLVMRuntime;
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
    let object = read(&mut file)?;
    let mut runtime : Box<dyn Runtime> = Box::new(LLVMRuntime::new());
    runtime.load(object)?;
    Ok(runtime.run())
}
