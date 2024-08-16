use ast::AstBuilder;
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

pub fn build(source_path : PathBuf, _output_path : PathBuf) -> Result<()> {
    let source = SourceBuffer::open(source_path)?;
    match AstBuilder::from(Lexer::new(&source)).parse_program() {
        Ok(p) => {dbg!(p); ()},
        Err(e) => return Err(e.into())
    
    }
    /*for t in Lexer::new(&source) {
        dbg!(t?);
    }*/
    todo!();
}
