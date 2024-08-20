use clap::ArgMatches;
use crate::editor::Editor;
use anyhow::Result;

pub struct Application {
    editor: Editor,
}

impl Application {
    pub fn new(args: ArgMatches) -> Result<Self> {
        let mut editor = Editor::new()?;
        let app = Self { editor };
        Ok(app)
    }
    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
