use actix_web::{HttpResponse, Responder, web};

pub async fn get(path: web::Path<(u64,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, your id is: {}", path.into_inner().0))
}