use clap::Args;
use clio::ClioPath;

#[derive(Args)]
pub struct CheckArgs {
    /// path to the file to run
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    pub file : ClioPath,
}
