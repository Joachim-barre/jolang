use core::fmt;
use std::fmt::Debug;

use super::IrObject;

pub fn write_ir(format : &mut std::fmt::Formatter, ir : &IrObject) -> fmt::Result {
    for f in ir.ext_fn.iter() {
        write!(format, "extern fn {} ({}) {}\n", f.0, "i64 ".repeat(f.1 as usize), if f.2 { "-> i64" } else { "" } )?;
    }

    write!(format, "fn main () -> i64 {{\n")?;
    for b in ir.blocks.iter().map(|b| b.take()).enumerate() {
        write!(format, ".B{}({}) : \n", b.0, "i64 ".repeat(b.1.argc as usize))?;
        for i in b.1.instructions.iter() {
            write!(format, "\t")?;
            match i {
                super::instructions::Instruction::Ret() => write!(format, "ret"),
                _ => todo!()
            }?;
        }
    }
    write!(format, "}}")?;
    Ok(())
}

impl Debug for IrObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_ir(f, self)
    }
}
