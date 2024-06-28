use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    discord_id: u64,
}
pub async fn get(path: web::Path<Params>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, your id is: {}", path.discord_id))
}