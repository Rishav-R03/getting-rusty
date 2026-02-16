use thiserror::Error;
use std::io;

#[derive(Error,Debug)]
pub enum EdmsError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Folder structure broken: {0}")]
    StructureBroken(String),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

