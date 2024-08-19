use anyhow::Result;
use crate::args::Args;
use crate::editor::Editor;

pub struct Application {
    editor: Editor
}

impl Application {
    pub fn new(args: Args) -> Result<Self> {
        let mut editor = Editor::new()?;
        let app = Self {
            editor
        };
        Ok(app)
    }
    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}