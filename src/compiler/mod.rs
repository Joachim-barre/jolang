use clio::OutputPath;
use std::fs::File;
use std::result::Result;
use crate::cli::compile::CompileArgs;

pub fn compile<'a>(args : CompileArgs) -> Result<(),()> {
    if !args.file.is_local() {
        println!("please input a local file");
        return Err(())
    }
    let mut _file : &File;
    match args.file.clone().open() {
        Ok(mut input) => {
            match input.get_file() {
                Some(f) => {
                    _file = &f;
                },
                None => {
                    println!("can't open file");
                    return Err(())
                }
            }
        },
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
    todo!();
}
