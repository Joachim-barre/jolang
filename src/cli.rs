use clap::{Parser, Subcommand};
use crate::run::RunArgs;
use crate::compile::CompileArgs;
use crate::show::ShowArgs;
use crate::check::CheckArgs;

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
    Compile(CompileArgs),
    /// run objects
    Run(RunArgs),
    /// print the ir
    Show(ShowArgs),
    /// check if the ir as no problems
    Check(CheckArgs)
}
