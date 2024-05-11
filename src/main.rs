use actix_web::{App, HttpServer};

use crate::bash::encrypt_bash; // Add this line to import the `write` function

mod bash;
mod core;
mod server;

const ERR_MSG: &str = "
how to use this application\n
 encrypt your file:\t-e <file_path>
 start encrypt service:\t-s
"; // Specify the type for the `ERR_MSG` constant

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mode = std::env::args().nth(1).expect(ERR_MSG);
    match mode.as_str() {
        "-s" => {
            HttpServer::new(|| {
                App::new()
                    .service(server::encrypt_service::encrypt)
                    .service(server::encrypt_service::health)
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
        }
        "-e" => {
            let file_path = std::env::args().nth(2).expect(ERR_MSG);
            encrypt_bash::encrypt_file(&file_path);
            panic!("加密完成")
        }
        _ => {
            panic!("{}", ERR_MSG)
        }
    }
}
