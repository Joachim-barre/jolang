use clap::Parser;
pub mod compile;
mod cli;
pub mod run;
pub mod show;
use cli::{Cli, Commands};
use jolangc::build;
use std::{i32, path::PathBuf, process::exit, fs::OpenOptions};
use anyhow::{anyhow, Result};
use clio::OutputPath;
use jolang_runtime::run;
use jolang_shared::ir::reader::read;

fn main() -> Result<()>{
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile(args) => {
            if !args.file.is_local() {
                return Err(anyhow!("please input a local file"))
            }
            let path = PathBuf::from(args.file.as_os_str());
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
            println!("building {} to {}...", path.to_str().unwrap_or("error"), object_file);
            return build(path, PathBuf::from(object_file.path().as_os_str()));
        }
        Commands::Run(args) => {
            if !args.file.is_local() {
                return Err(anyhow!("please input a local file"))
            }
            let path = PathBuf::from(args.file.as_os_str());
            println!("loading {}...", path.to_str().unwrap_or("error"));
            let code = run(path)?;
            if code >= i32::MAX.into() {
                exit(i32::MAX)
            }else if code <= i32::MIN.into() {
                exit(i32::MIN)
            }else {
                exit(code as i32);
            }
        },
        Commands::Show(args) => {
            if !args.file.is_local() {
                return Err(anyhow!("please input a local file"))
            }
            let path = PathBuf::from(args.file.as_os_str());
            let mut file = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(path)?;
            let object = read(&mut file)?;
            println!("{:?}", object);
            Ok(())
        }
    }
}
