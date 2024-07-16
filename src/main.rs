use std::result::Result;
use clap::Parser;
mod cli;
use cli::Commands;
use std::fs::File;

fn main() -> Result<(),()>{
    let cli = cli::Cli::parse();
    match cli.command {
        Commands::Compile(args) => {
            if !args.file.is_local() {
                println!("please input a local file");
                return Err(())
            }
            let mut _file : &File;
            match args.file.open() {
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
            todo!();
        }
    }
}
