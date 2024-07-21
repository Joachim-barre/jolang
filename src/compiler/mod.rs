use clio::OutputPath;
use std::fs::{File, OpenOptions};
use std::result::Result;
use crate::cli::compile::CompileArgs;
mod source_file;
mod text_data;
use text_data::TextData;
mod instructions;
use source_file::SourceFile;

pub fn compile<'a>(args : CompileArgs) -> Result<(),String> {
    if !args.file.is_local() {
        return Err(String::from("please input a local file"))
    }
    let mut file : File;
    match OpenOptions::new().read(true).write(false).truncate(false).append(false).open(args.file.clone().as_os_str()) {
        Ok(mut f) => {
            file = f
        }
        Err(_) => {
            return Err(String::from("can't open file"))
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
                return Err(format!("failed to open output file : {}", new_path.to_string()).to_string())
            }
        }
    }
    println!("building {} to {}...", args.file, object_file);
    let mut source = SourceFile::from(file);
    source.find_headers()?;
    let text = TextData::try_from(&source);
    todo!();
}
