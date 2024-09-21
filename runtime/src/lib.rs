use std::{fs::OpenOptions, path::PathBuf};
use anyhow::Result;
use jolang_shared::ir::{reader::read, IrObject};
#[cfg(feature = "llvm")]
use llvm::LLVMRuntime;
#[cfg(feature = "llvm")]
mod llvm;

pub trait Runtime {
    fn new() -> Self
        where Self : Sized;
    fn run(&mut self, object : IrObject) -> Result<i64>;
}

pub fn run(file : PathBuf) -> Result<i64> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file)?;
    let object = read(&mut file)?;
    let mut runtime : Box<dyn Runtime> = Box::new(LLVMRuntime::new());
    Ok(runtime.run(object)?)
}
