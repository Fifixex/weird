use crossterm::{
    cursor, execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Result, Write};

pub struct Terminal {
    size: (u16, u16),
}

impl Terminal {
    pub fn new() -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let size = terminal::size()?;
        Ok(Self { size })
    }

    pub fn size(&self) -> Result<(u16, u16)> {
        terminal::size()
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
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}
