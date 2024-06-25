mod app;

use std::env;

#[tokio::main]
async fn main() {
    // init env
    dotenv::dotenv().ok();
    set_env_default_values();

    // init logs
    env_logger::init();

    // start the webserver
    app::start().await.expect("TODO: panic message");
}

fn set_env_default_values() {
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "expresscord=info,actix_web=info");
    }

    if env::var("ADDRESS").ok().is_none() {
        env::set_var("ADDRESS", "127.0.0.1");
    }

    if env::var("PORT").ok().is_none() {
        env::set_var("PORT", "8080");
    }
}