use std::io::{Seek, SeekFrom, Write};
use crate::VERSION;

use super::{instructions::Instruction, IrObject};

pub fn write<T>(ir : IrObject, target : &mut T) -> std::io::Result<()>
where T : Write + Seek {
    target.write("\0JOO".as_bytes())?;
    let version : [u8;3] = [
        VERSION.major as u8,
        VERSION.minor as u8,
        VERSION.patch as u8
    ];
    target.write(&version[..])?;
    target.write(&(ir.ext_fn.len() as u64).to_le_bytes())?;
    let ext_pos = target.stream_position()?;
    target.write(&[0][..])?;
    target.write(&(ir.blocks.len() as u64).to_le_bytes())?;
    let block_pos = target.stream_position()?;
    target.write(&[0][..])?;
    let pos = target.stream_position()?;
    target.seek(SeekFrom::Start(ext_pos))?;
    target.write(&pos.to_le_bytes())?;
    target.seek(SeekFrom::Start(pos))?;
    for f in ir.ext_fn {
        target.write(&(f.0.len() as u64).to_le_bytes())?;
        target.write(f.0.as_bytes())?;
        target.write(&[f.1][..])?;
        target.write(&[f.2 as u8][..])?;
    }
    let pos = target.stream_position()?;
    target.seek(SeekFrom::Start(block_pos))?;
    target.write(&pos.to_le_bytes())?;
    target.seek(SeekFrom::Start(pos))?;
    for blk in ir.blocks.iter() {
        target.write(&blk.borrow().instructions.len().to_le_bytes())?;
target.write(&[blk.borrow().argc][..])?;
        target.write(&(0u64).to_le_bytes())?;
    }
    target.seek(SeekFrom::Start(pos))?;
    for blk in ir.blocks {
        let entry_pos = target.stream_position()?;
        let instr_pos = target.seek(SeekFrom::End(0))?;
        for i in &blk.borrow().instructions {
            target.write(&[i.opcode().into()][..])?;
            match i {
                Instruction::Iconst(op)
                    | Instruction::Dupx(op)
                    => {
                        target.write(&op.to_le_bytes())?;
                        target.write(&[0x00;24])
                    },
                Instruction::Br(op)
                    | Instruction::Call(op)
                    => {
                        target.write(&op.to_le_bytes())?;
                        target.write(&[0x00;24])
                    },
                Instruction::Briz(b1, b2) => {
                    target.write(&b1.to_le_bytes())?;
                    target.write(&b2.to_le_bytes())?;
                    target.write(&[0x00;8])
                },
                _ => target.write(&[0x00;24][..])
            }?;
        }
        target.seek(SeekFrom::Start(entry_pos))?;
        target.seek_relative(9)?;
        target.write(&instr_pos.to_le_bytes())?;
    }
    Ok(())
}
