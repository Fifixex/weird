use args::{CliArgs, Command};
use anyhow::Result;
use clap::Parser;

mod files;

mod args;
mod util;

pub fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .compact()
        .init();

    let args = CliArgs::parse();

    match args.command {
        Command::Files(args) => files::run(args)
    }
}