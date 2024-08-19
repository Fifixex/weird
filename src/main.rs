use anyhow::Result;
use clap::Parser;
use application::Application;
use args::Args;

mod files;

mod application;
mod args;
mod editor;
mod util;

pub fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();
    let mut app = Application::new(args)?;
    app.run()?;
    Ok(())
}