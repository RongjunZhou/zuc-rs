use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use log::{info, error};

use crate::bash::encrypt_bash;
use crate::config::ZucConfig;
use crate::error::{ZucError, ZucResult};
use crate::server::{encrypt, health, get_config};

mod core;
mod server;
mod bash;
mod config;
mod error;

const USAGE: &str = "
ZUC Encryption Tool

Usage:
  zuc -s                    Start encryption web service
  zuc -e <file_path>        Encrypt a file
  zuc -d <file_path>        Decrypt a file (same as encryption for ZUC)
  zuc --help                Show this help message

Environment Variables:
  ZUC_HOST                  Server host (default: 127.0.0.1)
  ZUC_PORT                  Server port (default: 8080)
";

#[actix_web::main]
async fn main() -> ZucResult<()> {
    // Initialize logging
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage_and_exit();
    }
    
    match args[1].as_str() {
        "-s" => {
            start_server().await?;
        }
        "-e" => {
            if args.len() < 3 {
                return Err(ZucError::InvalidInput("File path is required for encryption".to_string()));
            }
            encrypt_file(&args[2])?;
        }
        "-d" => {
            if args.len() < 3 {
                return Err(ZucError::InvalidInput("File path is required for decryption".to_string()));
            }
            decrypt_file(&args[2])?;
        }
        "--help" | "-h" => {
            println!("{}", USAGE);
        }
        _ => {
            print_usage_and_exit();
        }
    }
    
    Ok(())
}

async fn start_server() -> ZucResult<()> {
    let config = ZucConfig::from_env()?;
    let config = Arc::new(config);
    
    info!("Starting ZUC encryption server on {}", config.server_address());
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(encrypt)
            .service(health)
            .service(get_config)
    })
    .bind(config.server_address())?
    .run()
    .await
    .map_err(|e| ZucError::WebServiceError(e.to_string()))
}

fn encrypt_file(file_path: &str) -> ZucResult<()> {
    info!("Encrypting file: {}", file_path);
    
    match encrypt_bash::encrypt_file(file_path) {
        Ok(()) => {
            info!("File encrypted successfully: {}", file_path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to encrypt file {}: {}", file_path, e);
            Err(e)
        }
    }
}

fn decrypt_file(file_path: &str) -> ZucResult<()> {
    info!("Decrypting file: {}", file_path);
    
    match encrypt_bash::decrypt_file(file_path) {
        Ok(()) => {
            info!("File decrypted successfully: {}", file_path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to decrypt file {}: {}", file_path, e);
            Err(e)
        }
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("{}", USAGE);
    std::process::exit(1);
}
