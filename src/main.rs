mod app;

use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    app::start().await.expect("TODO: panic message");
}
