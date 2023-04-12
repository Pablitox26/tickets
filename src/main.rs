mod api;
mod models;
mod repository;

use std::env;

//modify imports below
use actix_web::{web::Data, App, HttpServer, middleware::Logger};
use repository::mongodb::MongoRepo;

use crate::api::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init().unwrap();
    
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    // building address and ip
    let port = std::env::var("PORT_API").unwrap_or("8080".to_string());
    let host = std::env::var("HOST_API").unwrap_or("127.0.0.1".to_string());
    let address = format!("{}:{}", host, port);
    println!("🚀 Server started successfully on => {}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .configure(handler::config)
        })
        .bind(&address)
        .unwrap_or_else(|err| {
            panic!(
                "🔥🔥🔥 Couldn't start the server in port {}: {:?}",
                port, err
            )
        })
        .run()
        .await
}
