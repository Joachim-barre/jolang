use ast::AstBuilder;
use lexer::Lexer;
use clio::OutputPath;
use source_buffer::SourceBuffer;
use anyhow::{anyhow, Result};
use crate::cli::compile::CompileArgs;
pub mod source_buffer;
pub mod lexer;
pub mod compiler_error;
pub mod source_span;
pub mod ast;
pub mod source_reader;

pub fn compile<'a>(args : CompileArgs) -> Result<()> {
    if !args.file.is_local() {
        return Err(anyhow!("please input a local file"))
    }
    let source : SourceBuffer;
    match SourceBuffer::open(args.file.as_os_str().into()) {
        Ok(s) => {
            source = s
        }
        Err(_) => {
            return Err(anyhow!("can't open file"))
        }
    }
    let mut object_file = match args.object_file {
        Some(p) => p,
        None => OutputPath::std()
    };
    if !object_file.is_local()  {
        let mut new_path = args.file.clone();
        new_path.set_extension("joo");
        object_file = match OutputPath::new(new_path.clone()) {
            Ok(path) => path,
            Err(_) => {
                return Err(anyhow!("failed to open output file : {}", new_path))
            }
        }
    }
    println!("building {} to {}...", &source.path.to_str().unwrap_or("error"), object_file);
    match AstBuilder::from(&mut Lexer::new(&source)).parse_program() {
        Ok(p) => {dbg!(p); ()},
        Err(e) => return Err(e.into())
    }
    /*for t in Lexer::new(&source) {
        dbg!(t?);
    }*/
    todo!();
}
