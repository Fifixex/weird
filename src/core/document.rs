use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Document {
    pub path: Option<PathBuf>,
    pub content: Vec<String>,
    pub is_modified: bool,
}

impl Document {
    pub fn open(path: PathBuf) -> Result<Self> {
        let document = fs::read_to_string(&path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        let content = document.lines().map(String::from).collect();

        Ok(Self {
            path: Some(path),
            content,
            is_modified: false,
        })
    }

    pub fn save(&self) -> Result<()> {
        if let Some(ref path) = self.path {
            fs::write(path, self.content.join("\n"))
                .with_context(|| format!("Failed to save file: {}", path.display()))?;
        }
        Ok(())
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.content.get(index).map(|s| s.as_str())
    }

    pub fn line_count(&self) -> usize {
        self.content.len()
    }

    pub fn lines_iter(&self, range: std::ops::Range<usize>) -> impl Iterator<Item = &str> {
        self.content[range].iter().map(|line| line.as_str())
    }

    pub fn insert_char(&mut self, row: usize, col: usize, ch: char) -> Result<()> {
        if row >= self.content.len() {
            self.content.push(String::new());
        }

        let line = &mut self.content[row];
        if col > line.len() {
            line.push_str(&" ".repeat(col - line.len()));
        }
        line.insert(col, ch);
        self.is_modified = true;
        Ok(())
    }

    pub fn remove_line(&mut self, index: usize) -> Result<()> {
        if index < self.content.len() {
            self.content.remove(index);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Index out of bounds"))
        }
    }
}
