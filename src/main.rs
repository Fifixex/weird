use anyhow::Result;
use crossterm::event::EventStream;
use app::Application;
use args::Args;

fn main() -> Result<()> {
    let mut args = Args::parse_args().context("Could not parse arguments!")?;
    let help_message = format!("Weird - v{}", env!("CARGO_PKG_VERSION"));

    if args.get_information {
        print!("{}", help_message);
        std::process::exit(0);
    }

    let mut app = Application::new(args).context("Unable to create new application")?;
    let exit_code = app.run(&mut EventStream::new())?;
    std::process::exit(exit_code);
}
