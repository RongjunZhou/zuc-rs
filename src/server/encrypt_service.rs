use crate::core::algorithm::Algorithm::EEA;
use crate::core::cipher::Cipher;
use actix_multipart::Multipart;
use actix_web::web::Bytes;
use actix_web::{get, post, HttpResponse};
use futures::StreamExt;

#[post("/encrypt")]
async fn encrypt(mut payload: Multipart) -> HttpResponse {
    let ck: [u8; 16] = [
        0x17, 0x3d, 0x14, 0xba, 0x50, 0x03, 0x73, 0x1d, 0x7a, 0x60, 0x04, 0x94, 0x70, 0xf0, 0x0a,
        0x29,
    ];

    let count = 0x66035492_u32;
    let bearer = 0xf_u32;
    let direction = 0_u32;
    let length = 0xc1_u32;

    // encrypt
    let mut cipher = Cipher::new(&ck, count, bearer, direction, EEA);

    let mut encrypted_data = Vec::new(); // Create a vector to store the encrypted data

    while let Some(item) = payload.next().await {
        let mut field: actix_multipart::Field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let date = chunk.unwrap();
            encrypted_data.append(&mut cipher.encrypt(&bytes_to_u32(&date), length));
            // Encrypt the data and store it in the vector
        }
    }
    let encrypted_data_bytes: Vec<u8> = encrypted_data
        .into_iter()
        .flat_map(|s| s.to_be_bytes())
        .collect(); // Convert Vec<Vec<u32>> to Vec<u8>
    HttpResponse::Ok()
        .content_type("application/octet-stream") // Set appropriate content type
        .body(encrypted_data_bytes) // Return the encrypted data
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("I'm alive!")
}

fn bytes_to_u32(bytes: &Bytes) -> Vec<u32> {
    bytes
        .chunks(4)
        .map(|chunk| {
            let mut result = 0;
            for &b in chunk {
                result = (result << 8) | (b as u32);
            }
            result
        })
        .collect()
}
