use std::{fs::OpenOptions, path::PathBuf};
use anyhow::Result;
use jolang_shared::ir::reader::read;

pub fn run(file : PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file)?;
    dbg!(read(&mut file)?);
    todo!()
}
