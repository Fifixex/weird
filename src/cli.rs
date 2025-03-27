use clap::{crate_name, crate_version, value_parser, Arg, Command};
use std::path::PathBuf;

pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about("A terminal-based code editor inspired by (Neo)Vim")
        .long_about(
            "A terminal-based code editor in Rust, inspired by Vim for efficient coding workflows.",
        )
        .arg(
            Arg::new("FILE")
                .help("File(s) to edit / create. Use '-' for standard input")
                .num_args(1..)
                .value_parser(value_parser!(PathBuf)),
        )
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .max_term_width(100)
}
