use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

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
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self { documents: vec![] })
    }

    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let document = Document::open(path)?;
        self.documents.push(document);
        Ok(())
    }

    pub fn render(&self) {
        for (i, document) in self.documents.iter().enumerate() {
            for line in &document.content {
                println!("{} - {}", i + 1, line)
            }
        }
    }
}
