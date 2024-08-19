use anyhow::Result;

pub struct Editor {
    /// Storage of all the documents opened in the editor
    pub documents: Vec<Document>,
    /// Interface for writing to the terminal
    pub terminal: Terminal,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            documents: vec![],
            terminal: Terminal::new(),
        })
    }
}
