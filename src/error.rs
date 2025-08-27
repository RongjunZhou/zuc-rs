use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZucError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Invalid input data: {0}")]
    InvalidInput(String),
    
    #[error("Web service error: {0}")]
    WebServiceError(String),
}

pub type ZucResult<T> = Result<T, ZucError>;