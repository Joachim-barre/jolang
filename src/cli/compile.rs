use clap::Args;
use clio::{InputPath, ClioPath};

#[derive(Args)]
pub struct CompileArgs {
    /// path to the file to compile
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    pub file : ClioPath
}
