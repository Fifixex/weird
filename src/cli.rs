use std::path::PathBuf;
use std::env;
use once_cell::sync::Lazy;
use clap::{
    crate_name, crate_version, value_parser, Arg, Command,
};

static VERSION: Lazy<String> = Lazy::new(|| {
    env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| crate_version!().to_string())
});

pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(VERSION.as_str())
        .hide_possible_values(true)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .max_term_width(100)
        .about("A terminal-based code editor inspired in (Neo)Vim")
        .long_about("A terminal-based code editor in Rust, inspired by Vim for efficient coding workflows.")
        .arg(
            Arg::new("FILE")
                .help("File(s) to edit / create. Use '-' for standard input")
                .num_args(1..)
                .value_parser(value_parser!(PathBuf))
        )
}