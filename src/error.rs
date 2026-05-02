use std::io;

use thiserror::Error;

#[derive(Error,Debug)]
pub enum TkcError {
    #[error("file system error: {0}")]
    Io(#[from] io::Error),

    #[error("failed to parse todo files: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Todo with id {0} not found")]
    TodoNotFound(u32),

    #[error("could not determine home directory")]
    HomeDirNotFound,

    #[error("unexpected error: {0}")]
    Other(String)

}