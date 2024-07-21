use clio::OutputPath;
use std::fs::{File, OpenOptions};
use std::result::Result;
use crate::cli::compile::CompileArgs;
mod source_file;
use source_file::SourceFile;

pub fn compile<'a>(args : CompileArgs) -> Result<(),()> {
    if !args.file.is_local() {
        println!("please input a local file");
        return Err(())
    }
    let mut file : File;
    match OpenOptions::new().read(true).write(false).truncate(false).append(false).open(args.file.clone().as_os_str()) {
        Ok(mut f) => {
            file = f
        }
        Err(_) => {
            println!("can't open file");
            return Err(())
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
                println!("failed to open output file : {}", new_path);
                return Err(())
            }
        }
    }
    println!("building {} to {}...", args.file, object_file);
    let mut source = SourceFile::from(file);
    todo!();
}
