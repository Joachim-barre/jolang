use super::IrObject;
use std::io::{Read, Seek, SeekFrom};
use anyhow::{Result, Ok, anyhow};

pub fn read<T>(input : &mut T) -> Result<IrObject>
where T: Read + Seek {
    todo!()
}
