use clap::Args;
use clio::{ClioPath, OutputPath};

#[derive(Args)]
pub struct CompileArgs {
    /// path to the file to compile
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    pub file : ClioPath,
    /// path of the generated object path
    #[clap(short, long, value_parser)]
    pub object_file : Option<OutputPath>
}
