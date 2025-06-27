mod modules;

use std::env;
use actix_web::{App, HttpServer};
use modules::controllers::hello_controller::{hello, hello_json};

fn get_server_ip() -> String {
    env::var("CRYPTO_SERVER_IP").unwrap_or_else(|_| "http://127.0.0.1".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_json)
    })
    .bind(format!("{}:8080", get_server_ip()))?
    .run()
    .await
}