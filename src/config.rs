use serde::{Deserialize, Serialize};
use std::env;
use crate::error::{ZucError, ZucResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZucConfig {
    pub server: ServerConfig,
    pub encryption: EncryptionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub key: [u8; 16],
    pub count: u32,
    pub bearer: u32,
    pub direction: u32,
}

impl Default for ZucConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            encryption: EncryptionConfig {
                key: [
                    0x17, 0x3d, 0x14, 0xba, 0x50, 0x03, 0x73, 0x1d, 
                    0x7a, 0x60, 0x04, 0x94, 0x70, 0xf0, 0x0a, 0x29,
                ],
                count: 0x66035492,
                bearer: 0xf,
                direction: 0,
            },
        }
    }
}

impl ZucConfig {
    pub fn from_env() -> ZucResult<Self> {
        let mut config = Self::default();
        
        // Override with environment variables if present
        if let Ok(host) = env::var("ZUC_HOST") {
            config.server.host = host;
        }
        
        if let Ok(port) = env::var("ZUC_PORT") {
            config.server.port = port.parse()
                .map_err(|_| ZucError::InvalidConfig("Invalid port number".to_string()))?;
        }
        
        Ok(config)
    }
    
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}