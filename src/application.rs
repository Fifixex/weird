use crate::editor::Editor;
use anyhow::Result;
use clap::ArgMatches;
use std::path::PathBuf;

pub struct Application {
    editor: Editor,
}

impl Application {
    pub fn new(args: ArgMatches) -> Result<Self> {
        let mut editor = Editor::new()?;

        if let Some(files) = args.get_many::<PathBuf>("FILE") {
            for file in files {
                editor.open_file(file.clone())?;
            }
        }

        Ok(Self { editor })
    }
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.editor.render()?;
            if !self.editor.handle_input()? {
                break;
            }
        }
        Ok(())
    }
}
