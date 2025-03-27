// filepath: /weird/weird/tests/editor_test.rs
use crate::core::editor::Editor;
use crate::core::document::Document;
use anyhow::Result;
use std::path::PathBuf;

#[test]
fn test_open_file() -> Result<()> {
    let mut editor = Editor::new()?;
    let test_file = PathBuf::from("test_file.txt");

    std::fs::write(&test_file, "Line 1\nLine 2\nLine 3")?;

    editor.open_file(test_file.clone())?;

    assert_eq!(editor.documents.len(), 1);
    assert_eq!(editor.documents[0].content, vec!["Line 1", "Line 2", "Line 3"]);

    std::fs::remove_file(test_file)?;

    Ok(())
}

#[test]
fn test_handle_input() -> Result<()> {
    let mut editor = Editor::new()?;
    let test_file = PathBuf::from("test_file.txt");
    std::fs::write(&test_file, "Line 1\nLine 2\nLine 3")?;
    editor.open_file(test_file.clone())?;

    editor.cursor_position = 0;
    assert!(editor.handle_input().is_ok());

    std::fs::remove_file(test_file)?;

    Ok(())
}

#[test]
fn test_render() -> Result<()> {
    let mut editor = Editor::new()?;
    let test_file = PathBuf::from("test_file.txt");
    std::fs::write(&test_file, "Line 1\nLine 2\nLine 3")?;
    editor.open_file(test_file.clone())?;

    assert!(editor.render().is_ok());

    std::fs::remove_file(test_file)?;

    Ok(())
}
