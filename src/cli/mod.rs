pub mod compile;
pub mod run;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// compile sources into object files
    Compile(compile::CompileArgs),
    /// run objects
    Run(run::RunArgs)
}
