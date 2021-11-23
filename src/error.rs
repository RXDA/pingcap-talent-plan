use serde_json;
use sled;
use std::{io, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),

    #[error("UTF-8 error: {}", _0)]
    Utf8(#[from] FromUtf8Error),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Unexpected command type")]
    UnexpectedCommandType,
    /// Sled error
    #[error("sled error: {0}")]
    Sled(#[from] sled::Error),
    /// Error with a string message
    #[error("{0}")]
    StringError(String),
}

pub type Result<T> = std::result::Result<T, KvsError>;
