use actix_web::{App, HttpServer};

mod core;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(server::encrypt_service::encrypt)
            .service(server::encrypt_service::health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
