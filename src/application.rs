use crate::core::editor::Editor;
use crate::terminal::events::{EditorAction, EventHandler};
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::path::PathBuf;

pub struct Application {
    editor: Editor,
    event_handler: EventHandler,
}

impl Application {
    pub fn new(args: ArgMatches) -> Result<Self> {
        let editor = Editor::new().context("Failed to initialize editor")?;
        let event_handler = EventHandler::default();

        let mut app = Self {
            editor,
            event_handler,
        };
        app.load_files_from_args(&args)?;
        Ok(app)
    }

    fn load_files_from_args(&mut self, args: &ArgMatches) -> Result<()> {
        if let Some(files) = args.get_many::<PathBuf>("FILE") {
            for file in files {
                self.editor
                    .open_file(file.clone())
                    .with_context(|| format!("Failed to open file: {}", file.display()))?;
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.editor.render().context("Failed to render editor")?;

            match self
                .event_handler
                .handle_input()
                .context("Failed to handle input")?
            {
                EditorAction::Quit => break,
                EditorAction::MoveCursor(direction) => {
                    self.editor
                        .move_cursor(direction)
                        .context("Failed to move cursor")?;
                }
                EditorAction::Continue => {}
            }
        }
        Ok(())
    }
}
