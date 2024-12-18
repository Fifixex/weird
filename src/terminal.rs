use crossterm::{
    cursor, execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Result;
use std::io::{stdout, Write};

pub struct Terminal {
    pub size: (u16, u16),
}

impl Terminal {
    pub fn new() -> Self {
        let (cols, rows) = terminal::size().unwrap_or((80, 24));
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
        Self { size: (cols, rows) }
    }

    pub fn clear_screen(&self) -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))
    }

    pub fn set_cursor_position(&self, x: u16, y: u16) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(x, y))
    }

    pub fn flush(&self) -> Result<()> {
        stdout().flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}
