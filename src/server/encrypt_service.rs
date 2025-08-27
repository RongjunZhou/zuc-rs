use crate::core::algorithm::Algorithm::EEA;
use crate::core::cipher::Cipher;
use crate::config::ZucConfig;
use crate::error::{ZucError, ZucResult};
use actix_multipart::Multipart;
use actix_web::web::Bytes;
use actix_web::{get, post, HttpResponse, web};
use futures::StreamExt;
use std::sync::Arc;

#[post("/encrypt")]
async fn encrypt(
    payload: Multipart,
    config: web::Data<Arc<ZucConfig>>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = encrypt_data(payload, &config).await;
    
    match result {
        Ok(encrypted_data) => {
            Ok(HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(encrypted_data))
        }
        Err(e) => {
            log::error!("Encryption error: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "error": "Encryption failed",
                    "message": e.to_string()
                })))
        }
    }
}

async fn encrypt_data(
    mut payload: Multipart,
    config: &ZucConfig,
) -> ZucResult<Vec<u8>> {
    let mut encrypted_data = Vec::new();
    let mut cipher = Cipher::new(
        &config.encryption.key,
        config.encryption.count,
        config.encryption.bearer,
        config.encryption.direction,
        EEA,
    )?;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ZucError::WebServiceError(e.to_string()))?;
        
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| ZucError::WebServiceError(e.to_string()))?;
            let u32_data = bytes_to_u32(&data);
            
            if !u32_data.is_empty() {
                let encrypted = cipher.encrypt(&u32_data, u32_data.len() as u32)?;
                encrypted_data.extend(encrypted.into_iter().flat_map(|s| s.to_be_bytes()));
            }
        }
    }

    Ok(encrypted_data)
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "zuc-encryption",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[get("/config")]
async fn get_config(config: web::Data<Arc<ZucConfig>>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "server": {
            "host": config.server.host,
            "port": config.server.port
        },
        "encryption": {
            "count": config.encryption.count,
            "bearer": config.encryption.bearer,
            "direction": config.encryption.direction
        }
    }))
}

fn bytes_to_u32(bytes: &Bytes) -> Vec<u32> {
    bytes
        .chunks(4)
        .map(|chunk| {
            let mut result = 0u32;
            for &b in chunk {
                result = (result << 8) | (b as u32);
            }
            result
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::App;

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(
            App::new()
                .service(health)
        ).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }
}
