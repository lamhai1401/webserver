use std::env;
use tokio::task;

mod app;
mod db;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    app::start().await;

    Ok(())
}
