use std::result::Result;
use clap::Parser;
mod cli;
use cli::Commands;
mod compiler;

fn main() -> Result<(),()>{
    let cli = cli::Cli::parse();
    match cli.command {
        Commands::Compile(args) => {
            match compiler::compile(args) {
                Ok(_) => return Ok(()),
                Err(desc) => {
                    println!("{}", desc);
                    return Err(())
                }
            }
        }
    }
}
