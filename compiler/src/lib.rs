use ast::AstBuilder;
use generator::{Generate, IrGenerator};
use lexer::Lexer;
use source_buffer::SourceBuffer;
use anyhow::Result;
use std::path::PathBuf;
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
            dbg!(generator.into_ir());
            Ok(())
        },
        Err(e) => return Err(e.into())
    }
}
