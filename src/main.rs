use std::hint::black_box;
use anyhow::Result;
use clap::Parser;
mod cli;
use cli::Commands;
mod compiler;
mod commons;
mod runtime;
use runtime::externs::PRINT_INT;

fn main() -> Result<()>{
    black_box(PRINT_INT);
    let cli = cli::Cli::parse();
    match cli.command {
        Commands::Compile(args) => {
            return compiler::compile(args);
        }
        Commands::Run(args) => {
            return runtime::run(args);
        }
    }
}
