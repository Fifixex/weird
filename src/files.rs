use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command()]
pub struct FilesArgs {
    input: std::path::PathBuf,
}

pub fn run(args: FilesArgs) -> Result<()> {
    let path = args.input;
    Ok(())
}
