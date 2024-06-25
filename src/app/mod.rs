use std::env;
use log;
use actix_web::{
    HttpServer,
    App,
    middleware::Logger
};

use actix_web::{get, HttpResponse, Responder};
use log::info;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
pub async fn start() -> std::io::Result<()> {
    // get the env
    let address = env::var("ADDRESS").ok().expect("Expected a valid binding address for ADDRESS");
    let port: u16 = env::var("PORT").ok().expect("Expected a valid port for PORT")
        .parse().ok().expect(format!("Invalid port: {:?}", env::var("PORT")
        .ok().expect("Invalid port.")).as_str());

    // start the server
    info!("Starting expresscord on {:?}:{:?}", address, port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
    })
        .bind((address, port))?
        .run()
        .await
}