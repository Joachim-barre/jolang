use core::fmt;
use std::cell::RefCell;
use std::fmt::Debug;
use super::block::Block;
use super::instructions::Instruction;
use super::IrObject;

pub fn write_ir(format : &mut std::fmt::Formatter, ir : &IrObject) -> fmt::Result {
    for f in ir.ext_fn.iter() {
        write!(format, "extern fn {} ({}) {}\n", f.0, "i64 ".repeat(f.1 as usize), if f.2 { "-> i64" } else { "" } )?;
    }

    write!(format, "fn main () -> i64 {{\n")?;
    let mut new_blocks = Vec::new();
    for b in ir.blocks.iter().map(|b| b.take()).enumerate() {
        write!(format, ".B{}({}) : \n", b.0, "i64 ".repeat(b.1.argc as usize))?;
        for i in b.1.instructions.iter() {
            write!(format, "\t")?;
            match i {
                Instruction::Ret() => write!(format, "ret"),
                Instruction::Reti() => write!(format, "reti"),
                Instruction::Iconst(s, v) => write!(format, "iconst i{} {}", s, v),
                Instruction::Br(b) => write!(format, "br B{}", b),
                Instruction::Dup() => write!(format, "dup"),
                Instruction::Dupx(v) => write!(format, "dupx {}", v),
                Instruction::Swap() => write!(format, "swap"),
                Instruction::Call(id) => write!(format, "call {}", id),
                Instruction::Neg() => write!(format, "neg"),
                Instruction::Add() => write!(format, "add"),
                Instruction::Sub() => write!(format, "sub"),
                Instruction::Mul() => write!(format, "mul"),
                Instruction::Div() => write!(format, "div"),
                Instruction::Eq() => write!(format, "eq"),
                Instruction::Ne() => write!(format, "ne"),
                Instruction::Gt() => write!(format, "gt"),
                Instruction::Ge() => write!(format, "ge"),
                Instruction::Le() => write!(format, "le"),
                Instruction::Lt() => write!(format, "lt"),
                Instruction::Lsh() => write!(format, "lsh"),
                Instruction::Rsh() => write!(format, "rsh"),
                Instruction::Briz(b1, b2) => write!(format, "briz {} {}", b1, b2)
            }?;
            write!(format, "\n")?;
        }
        new_blocks.push(b.1);
    }
    ir.blocks.iter()
        .zip(new_blocks)
        .for_each(|(cell, b)| {
            cell.replace(b);
        });
    write!(format, "}}")?;
    Ok(())
}

impl Debug for IrObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_ir(f, self)
    }
}
