use crate::{ir::{block::Block, instructions::{Opcodes, Instruction}}, VERSION, VERSION_STR};
use super::IrObject;
use std::{cell::RefCell, io::{Read, Seek, SeekFrom}};
use anyhow::{anyhow, Context, Ok, Result};
use index_list::IndexList;

pub fn read<T>(input : &mut T) -> Result<IrObject>
where T: Read + Seek {
    let mut header : [u8;4] = [0;4];
    input.read_exact(&mut header[..])?;
    if header[..] != "\0JOO".as_bytes()[..] {
        return Err(anyhow!("bad header"))
    }
    let mut version : [u8;3] = [0;3];
    input.read_exact(&mut version[..])?;
    if version[0] as u64 != VERSION.major || version[1] as u64 > VERSION.minor {
        return Err(anyhow!("unsupported version : {}.{}.{}", version[0], version[1], version[2]))
            .context(format!("the program version is {}", VERSION_STR))
    }
    let mut tables : [u8;32] = [0;32];
    input.read_exact(&mut tables[..])?;
    let ext_fn_count = u64::from_le_bytes(tables[0..8].try_into()?);
    let ext_fn_pos = u64::from_le_bytes(tables[8..16].try_into()?);
    let block_count = u64::from_le_bytes(tables[16..24].try_into()?);
    let block_pos = u64::from_le_bytes(tables[24..32].try_into()?);
    let mut object = IrObject::new();
    object.ext_fn.reserve(ext_fn_count as usize);
    object.blocks.reserve(block_count as usize);
    input.seek(SeekFrom::Start(ext_fn_pos))?;
    for _ in 0..ext_fn_count {
        let mut buffer : [u8;4] = [0;4];
        input.read_exact(&mut buffer)?;
        let name_size = u32::from_le_bytes(buffer);
        let mut name_buffer = vec![0;name_size as usize];
        input.read_exact(&mut name_buffer[..])?;
        let name = String::from_utf8(name_buffer)?;
        let mut info_buf = [0;2];
        input.read_exact(&mut info_buf)?;
        let argc = info_buf[0];
        let returns = info_buf[1] != 0;
        object.ext_fn.push((name, argc, returns));
    }
    input.seek(SeekFrom::Start(block_pos))?;
    for _ in 0..block_count {
        let mut buffer : [u8;16] = [0;16];
        input.read_exact(&mut buffer)?;
        let block_size = u64::from_le_bytes(buffer[..8].try_into()?);
        let block_argc = u64::from_le_bytes(buffer[8..16].try_into()?);
        let mut block_sizes = Vec::new();
        block_sizes.reserve(block_argc as usize);
        for _ in 0..block_argc {
            let mut buffer : [u8;8] = [0;8];
            input.read_exact(&mut buffer);
            block_sizes.push(u64::from_le_bytes(buffer));
        }
        let mut buffer = [0;8];
        input.read_exact(&mut buffer);
        let block_pos = u64::from_le_bytes(buffer[..].try_into()?);
        let mut block = Block::new(block_sizes);
        let mut instructions = Vec::new();
        instructions.reserve(block_size as usize);
        let next_pos = input.stream_position()?;
        input.seek(SeekFrom::Start(block_pos))?;
        for _ in 0..block_size {
            let mut buffer = [0;32];
            input.read_exact(&mut buffer)?;
            let opcode : Opcodes = buffer[7].try_into()?;
            instructions.push(match opcode {
                Opcodes::Ret => Instruction::Ret(),
                Opcodes::Reti => Instruction::Reti(),
                Opcodes::Iconst => {
                    let size = u64::from_le_bytes(buffer[8..16].try_into()?);
                    let value = u128::from_le_bytes(buffer[16..32].try_into()?);
                    Instruction::Iconst(size, value)
                },
                Opcodes::Br => {
                    let block = u64::from_le_bytes(buffer[8..16].try_into()?);
                    Instruction::Br(block)
                },
                Opcodes::Dup => Instruction::Dup(),
                Opcodes::Dupx 
                    | Opcodes::Icast => {
                    let offset = u64::from_le_bytes(buffer[8..16].try_into()?);
                    Instruction::Dupx(offset)
                },
                Opcodes::Swap => Instruction::Swap(),
                Opcodes::Call => {
                    let id = u64::from_le_bytes(buffer[8..16].try_into()?);
                    Instruction::Call(id)
                },
                Opcodes::Neg => Instruction::Neg(),
                Opcodes::Add => Instruction::Add(),
                Opcodes::Sub => Instruction::Sub(),
                Opcodes::Mul => Instruction::Mul(),
                Opcodes::Div => Instruction::Div(),
                Opcodes::Eq => Instruction::Eq(),
                Opcodes::Ne => Instruction::Ne(),
                Opcodes::Gt => Instruction::Gt(),
                Opcodes::Ge => Instruction::Ge(),
                Opcodes::Le => Instruction::Le(),
                Opcodes::Lt => Instruction::Lt(),
                Opcodes::Lsh => Instruction::Lsh(),
                Opcodes::Rsh => Instruction::Rsh(),
                Opcodes::Briz => {
                    let b1 = u64::from_le_bytes(buffer[8..16].try_into()?);
                    let b2 = u64::from_le_bytes(buffer[16..24].try_into()?);
                    Instruction::Briz(b1, b2)
                },
                _ => return Err(anyhow!("bad opcode {}", buffer[7]))
            })
        }
        block.instructions = IndexList::from_iter(instructions.into_iter());
        object.blocks.push(block);
        input.seek(SeekFrom::Start(next_pos))?;
    }

    Ok(object) 
}
