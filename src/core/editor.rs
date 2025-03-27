use crate::core::document::Document;
use crate::terminal::events::Direction;
use crate::terminal::ui::Terminal;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct Editor {
    documents: Vec<Document>,
    terminal: Terminal,
    cursor_position: Position,
    active_document: usize,
    scroll_offset: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub preferred_x: usize,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new().context("Failed to initialize terminal")?;

        Ok(Self {
            terminal,
            documents: Vec::new(),
            cursor_position: Position::default(),
            active_document: 0,
            scroll_offset: 0,
        })
    }

    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let document = Document::open(path)?;
        self.documents.push(document);
        Ok(())
    }

    pub fn move_cursor(&mut self, direction: Direction) -> Result<()> {
        if let Some(document) = self.documents.get(self.active_document) {
            let mut new_position = self.cursor_position;

            match direction {
                Direction::Up => {
                    new_position.y = new_position.y.saturating_sub(1);
                    self.adjust_cursor_x(&mut new_position, document);
                }
                Direction::Down => {
                    if new_position.y < document.line_count().saturating_sub(1) {
                        new_position.y += 1;
                        self.adjust_cursor_x(&mut new_position, document);
                    }
                }
                Direction::Left => {
                    if new_position.x > 0 {
                        new_position.x -= 1;
                    } else if new_position.y > 0 {
                        new_position.y -= 1;
                        if let Some(line) = document.get_line(new_position.y) {
                            new_position.x = line.len();
                        }
                    }
                }
                Direction::Right => {
                    if let Some(line) = document.get_line(new_position.y) {
                        if new_position.x < line.len() {
                            new_position.x += 1;
                        } else if new_position.y < document.line_count().saturating_sub(1) {
                            new_position.y += 1;
                            new_position.x = 0;
                        }
                    }
                }
            }

            new_position.preferred_x = new_position.x;
            self.cursor_position = new_position;
            self.scroll_into_view()?;
        }
        Ok(())
    }

    fn adjust_cursor_x(&self, position: &mut Position, document: &Document) {
        if let Some(line) = document.get_line(position.y) {
            position.x = position.preferred_x.min(line.len());
        }
    }

    fn scroll_into_view(&mut self) -> Result<()> {
        let (_, height) = self.terminal.size()?;
        let visible_rows = height as usize - 2;

        if self.cursor_position.y < self.scroll_offset {
            self.scroll_offset = self.cursor_position.y;
        } else if self.cursor_position.y >= self.scroll_offset + visible_rows {
            self.scroll_offset = self.cursor_position.y - visible_rows + 1;
        }
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        self.terminal
            .clear_screen()
            .context("Failed to clear screen")?;

        if let Some(document) = self.documents.get(self.active_document) {
            self.render_document(document)?;
        }

        self.terminal
            .flush()
            .context("Failed to flush terminal buffer")?;
        Ok(())
    }

    fn render_document(&self, document: &Document) -> Result<()> {
        let visible_lines = self.terminal.size()?.1 as usize;
        let start_line = self.cursor_position.y.saturating_sub(visible_lines / 2);
        let end_line = (start_line + visible_lines).min(document.line_count());

        for (i, line) in document.lines_iter(start_line..end_line).enumerate() {
            self.terminal
                .set_cursor_position(0, i as u16)
                .context("Failed to set cursor position")?;

            let prefix = if i + start_line == self.cursor_position.y {
                "> "
            } else {
                "  "
            };

            print!("{}{}", prefix, line);
        }
        Ok(())
    }
}
