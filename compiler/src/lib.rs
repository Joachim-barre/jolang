use ast::AstBuilder;
use generator::{Generate, IrGenerator};
use lexer::Lexer;
use source_buffer::SourceBuffer;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use jolang_shared::ir::writer::write;
pub mod source_buffer;
pub mod lexer;
pub mod compiler_error;
pub mod source_span;
pub mod ast;
pub mod source_reader;
pub mod generator;
pub mod scope;

pub fn build(source_path : PathBuf, _output_path : PathBuf) -> Result<()> {
    let source = SourceBuffer::open(source_path)?;
    match AstBuilder::from(Lexer::new(&source)).parse_program() {
        Ok(p) => {
            let mut generator = IrGenerator::new();
            p.generate(&mut generator);
            let mut obj_file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(_output_path)?;
            write(generator.into_ir(), &mut obj_file)?;
            Ok(())
        },
        Err(e) => return Err(e.into())
    }
}
