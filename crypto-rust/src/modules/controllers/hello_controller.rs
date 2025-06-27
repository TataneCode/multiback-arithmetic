use actix_web::{get, HttpResponse, Responder};
use crate::modules::responses::hello_response::HelloResponse;

#[get("/hello")]
pub async fn hello() -> impl Responder {
    "Hello, API!"
}

#[get("/json")]
pub async fn hello_json() -> impl Responder {
    let response = HelloResponse {
        message: "Hello, JSON!".to_string(),
        status: 200,
        data: Some("Additional data here".to_string()),
    };
    HttpResponse::Ok().json(response)
}