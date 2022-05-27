use std::env;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("Hello, world!");
    let hai = env::var("HAI").ok().unwrap();
    println!("{}", hai);
}
