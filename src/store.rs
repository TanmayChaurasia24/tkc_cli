use std::fs;
use std::path::PathBuf;

use crate::error::TkcError;
use crate::todo::Todo;

fn data_dir() -> Result<PathBuf, TkcError> {
    let home = dirs::home_dir().ok_or(TkcError::HomeDirNotFound)?;

    Ok(home.join(".tkc"))
}

fn todo_path() -> Result<PathBuf, TkcError> {
    Ok(data_dir()?.join("todos.json"))
}

pub fn load() -> Result<Vec<Todo>, TkcError> {
    let path = todo_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)?;

    let todos: Vec<Todo> = serde_json::from_str(&content)?;

    Ok(todos)
}

pub fn save(todos: &[Todo]) -> Result<(), TkcError> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir)?;

    let path = todo_path()?;

    let contents = serde_json::to_string_pretty(todos)?;
    fs::write(&path, contents)?;

    Ok(())
}

pub fn next_id(todos: &[Todo]) -> u32 {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}