use std::{fs::OpenOptions, path::PathBuf};
use anyhow::Result;
use jolang_shared::ir::{reader::read, IrObject};
use platforms::Platform;
#[cfg(feature = "llvm")]
mod llvm;
mod platforms;

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
    let platform = platforms::CurrentPlatform::new();
    let mut runtime : Box<dyn Runtime> = platform.default_runtime();
    Ok(runtime.run(object)?)
}
