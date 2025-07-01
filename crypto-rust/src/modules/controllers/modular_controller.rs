use crate::modules::requests::modular_request::CongruentQuery;
use crate::modules::responses::modular_response::Congruent;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/congruent")]
pub async fn congruent(query: web::Query<CongruentQuery>) -> impl Responder {
    let n1_remaining = query.n1 % query.modulo;
    let n2_remaining = query.n2 % query.modulo;
    let is_congruent = n1_remaining == n2_remaining;

    let response = Congruent {
        n1_remaining,
        n2_remaining,
        is_congruent,
    };

    HttpResponse::Ok().json(response)
}
