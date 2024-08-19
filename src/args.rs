use clap::{Parser, Subcommand};

use crate::files;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage files
    #[command(alias = "f")]
    Files(files::FilesArgs),
}
