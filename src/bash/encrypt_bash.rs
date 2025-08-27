use std::{fs::File, io::{Read, Write}};
use crate::core::algorithm::Algorithm::EEA;
use crate::core::cipher::Cipher;
use crate::error::{ZucError, ZucResult};
use crate::config::ZucConfig;

pub fn encrypt_file(file_path: &str) -> ZucResult<()> {
    encrypt_file_with_config(file_path, &ZucConfig::default())
}

pub fn encrypt_file_with_config(file_path: &str, config: &ZucConfig) -> ZucResult<()> {
    // Validate file path
    if file_path.trim().is_empty() {
        return Err(ZucError::InvalidFilePath("File path cannot be empty".to_string()));
    }
    
    // Read file content
    let mut file = File::open(file_path)
        .map_err(|e| ZucError::Io(e))?;
    
    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|e| ZucError::Io(e))?;
    
    if content.is_empty() {
        return Err(ZucError::InvalidInput("File is empty".to_string()));
    }
    
    // Create cipher and encrypt
    let mut cipher = Cipher::new(
        &config.encryption.key,
        config.encryption.count,
        config.encryption.bearer,
        config.encryption.direction,
        EEA
    )?;
    
    let encrypted = cipher.encrypt_bytes(&content, content.len() as u32)?;
    
    // Write encrypted content back to file
    let mut output_file = File::create(file_path)
        .map_err(|e| ZucError::Io(e))?;
    
    output_file.write_all(&encrypted)
        .map_err(|e| ZucError::Io(e))?;
    
    Ok(())
}

pub fn decrypt_file(file_path: &str) -> ZucResult<()> {
    // For ZUC algorithm, decryption is the same as encryption
    encrypt_file(file_path)
}

pub fn decrypt_file_with_config(file_path: &str, config: &ZucConfig) -> ZucResult<()> {
    // For ZUC algorithm, decryption is the same as encryption
    encrypt_file_with_config(file_path, config)
}