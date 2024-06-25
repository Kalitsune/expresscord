use actix_web::{
    HttpServer,
    App,
    middleware::Logger
};

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}