use crate::core::document::Document;
use anyhow::Result;
use std::path::PathBuf;

#[test]
fn test_open_document() -> Result<()> {
    let path = PathBuf::from("test_file.txt");
    let content = "Hello, world!\nThis is a test document.";
    std::fs::write(&path, content)?;

    let document = Document::open(path.clone())?;
    assert_eq!(document.content.len(), 2);
    assert_eq!(document.content[0], "Hello, world!");
    assert_eq!(document.content[1], "This is a test document.");

    std::fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_open_nonexistent_document() {
    let path = PathBuf::from("nonexistent_file.txt");
    let result = Document::open(path);
    assert!(result.is_err());
}