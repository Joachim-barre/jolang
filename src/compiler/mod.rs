use clio::OutputPath;
use std::fs::{File, OpenOptions};
use std::io::Seek;
use anyhow::{anyhow, Result};
use crate::cli::compile::CompileArgs;
use crate::commons::object::Object;
pub mod source_file;
pub mod text_data;
pub mod lexer;
use text_data::TextData;
use source_file::SourceFile;

pub fn compile<'a>(args : CompileArgs) -> Result<()> {
    if !args.file.is_local() {
        return Err(anyhow!("please input a local file"))
    }
    let file : File;
    match OpenOptions::new().read(true).write(false).truncate(false).append(false).open(args.file.clone().as_os_str()) {
        Ok(f) => {
            file = f
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
    println!("building {} to {}...", args.file, object_file);
    let mut source = SourceFile::from(file);
    source.find_headers()?;
    let _ = source.file.rewind();
    let text = TextData::try_from(&source)?;
    let _ = source.file.rewind();
    let data = source.parse_data()?;
    let object = Object::build(text, data)?;
    let mut object_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(object_file.path().as_os_str()) {
            Ok(f) => f,
            Err(_) => {
                return Err(anyhow!("failed to open object file"))
            }
    };
    object.save(&mut object_file)?;
    println!("success");
    Ok(())
}
