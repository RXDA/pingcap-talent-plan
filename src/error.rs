use std::io;
use serde_json;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Unexpected command type")]
    UnexpectedCommandType,
}


// impl From<serde_json::Error> for KvsError {
//     fn from(err: serde_json::Error) -> Self {
//         KvsError::Serde(err)
//     }
// }

pub type Result<T> = std::result::Result<T, KvsError>;