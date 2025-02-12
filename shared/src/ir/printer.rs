use core::fmt;
use std::cell::RefCell;
use std::fmt::Debug;
use super::block::Block;
use super::instructions::Instruction;
use super::IrObject;

pub fn write_ir(format : &mut std::fmt::Formatter, ir : &IrObject) -> fmt::Result {
    for f in ir.ext_fn.iter() {
        write!(format, "extern fn {} (", f.name)?;
        for (i, arg) in f.sig.args.iter().enumerate() {
            if i!=0{
                write!(format, ", ");
            }
            write!(format, "{}", arg);
        }
        write!(format, ") -> {};\n", f.sig.ret);
    }

    write!(format, "fn main () -> i64 {{\n")?;
    for (i, blk) in ir.blocks.iter().enumerate() {
        write!(format, "B{} : \n", i)?;
        for j in blk.instructions.iter() {
            write!(format, "\t")?;
            match j { 
                Instruction::Nop() => write!(format, "nop"),
                Instruction::Pop(size) => write!(format, "pop {}", size),
                Instruction::Dup(size) => write!(format, "dup {}", size),
                Instruction::Swap(size) => write!(format, "swap {}", size),
                Instruction::Br(id) => write!(format, "br {}", id),
                Instruction::Briz(id1, id2) => write!(format, "briz {} {}", id1, id2),
                Instruction::Call(id) => write!(format, "call {}", id),
                Instruction::Varref() => write!(format, "varref"),
                Instruction::Iconst(size, val) => write!(format, "iconst {} {}", size, val),
                Instruction::Iload(size) => write!(format, "iload {}", size),
                Instruction::Istore(size) => write!(format, "istore {}", size),
                Instruction::Iret(size) => write!(format, "iret {}", size),
                Instruction::Inot(size) => write!(format, "inot {}", size),
                Instruction::Ior(size) => write!(format, "ior {}", size),
                Instruction::Iand(size) => write!(format, "iand {}", size),
                Instruction::Ixor(size) => write!(format, "ixor {}", size),
                Instruction::Ilshr(size) => write!(format, "ilshr {}", size),
                Instruction::Iashr(size) => write!(format, "iashr {}", size),
                Instruction::Ishl(size) => write!(format, "ishl {}", size),
                Instruction::Ineg(size) => write!(format, "ineg {}", size),
                Instruction::Iadd(size) => write!(format, "iadd {}", size),
                Instruction::Isub(size) => write!(format, "isub {}", size),
                Instruction::Imul(size) => write!(format, "imul {}", size),
                Instruction::Idiv(size) => write!(format, "idiv {}", size),
                Instruction::Udiv(size) => write!(format, "udiv {}", size),
                Instruction::Irem(size) => write!(format, "irem {}", size),
                Instruction::Urem(size) => write!(format, "urem {}", size),
                Instruction::Ieq(size) => write!(format, "ieq {}", size),
                Instruction::Ine(size) => write!(format, "ine {}", size),
                Instruction::Ige(size) => write!(format, "ige {}", size),
                Instruction::Igt(size) => write!(format, "igt {}", size),
                Instruction::Uge(size) => write!(format, "uge {}", size),
                Instruction::Ugt(size) => write!(format, "ugt {}", size),
                Instruction::Ilt(size) => write!(format, "ilt {}", size),
                Instruction::Ile(size) => write!(format, "ile {}", size),
                Instruction::Ule(size) => write!(format, "ule {}", size),
                Instruction::Ult(size) => write!(format, "ult {}", size),
                Instruction::Iconv(size1, size2) => write!(format, "iconv {} {}", size1, size2),
                Instruction::Uconv(size1, size2) => write!(format, "uconv {} {}", size1, size2),
            }?;
            write!(format, "\n")?;
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
