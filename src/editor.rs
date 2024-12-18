use crate::terminal::Terminal;
use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode};
use std::{fs, path::PathBuf};

use std::io::{stdout, Write};

pub struct Document {
    pub path: Option<PathBuf>,
    pub content: Vec<String>,
}

impl Document {
    pub fn open(path: PathBuf) -> Result<Self> {
        let document = fs::read_to_string(&path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        let content = document.lines().map(String::from).collect();

        Ok(Self {
            path: Some(path),
            content,
        })
    }
}

pub struct Editor {
    /// Storage of all the documents opened in the editor
    pub documents: Vec<Document>,
    /// Interface for writing to the terminal
    pub terminal: Terminal,
    /// Current cursor position
    pub cursor_position: usize,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(),
            documents: vec![],
            cursor_position: 0,
        })
    }

    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let document = Document::open(path)?;
        self.documents.push(document);
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<bool> {
        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Char('j') => {
                        self.cursor_position = std::cmp::min(
                            self.cursor_position + 1,
                            self.documents[0].content.len() - 1,
                        );
                    }
                    KeyCode::Char('k') => {
                        self.cursor_position = self.cursor_position.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }

    pub fn render(&self) -> Result<()> {
        self.terminal.clear_screen()?;

        let visible_lines = self.terminal.size.1 as usize;
        let start_line = self.cursor_position.saturating_sub(visible_lines / 2);
        let end_line = std::cmp::min(start_line + visible_lines, self.documents[0].content.len());

        for (i, line) in self.documents[0].content[start_line..end_line]
            .iter()
            .enumerate()
        {
            self.terminal.set_cursor_position(0, i as u16)?;
            if i + start_line == self.cursor_position {
                write!(stdout(), "> {}", line)?;
            } else {
                write!(stdout(), "  {}", line)?;
            }
        }

        Ok(self.terminal.flush()?)
    }
}
