use std::result::Result;
use clap::Parser;
mod cli;
use cli::Commands;

fn main() -> Result<(),()>{
    let cli = cli::Cli::parse();
    match &cli.command {
        Commands::Compile(_args) => {
            todo!();
        }
    }
}
