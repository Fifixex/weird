use anyhow::Result;
use application::Application;
use cli::build_app;

mod application;
mod cli;
mod core;
mod terminal;

pub fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .compact()
        .init();

    let args = build_app().get_matches();
    let mut app = Application::new(args)?;
    app.run()?;
    Ok(())
}
