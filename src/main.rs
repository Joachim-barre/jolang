use std::{hint::black_box, result::Result};
use clap::Parser;
mod cli;
use cli::Commands;
mod compiler;
mod commons;
mod runtime;
use runtime::externs::PRINT_INT;

fn main() -> Result<(),()>{
    black_box(PRINT_INT);
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
        Commands::Run(args) => {
            match runtime::run(args) {
                Ok(_) => return Ok(()),
                Err(desc) => {
                    println!("{}", desc);
                    return Err(())
                }
            }
        }
    }
}
