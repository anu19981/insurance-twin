use actix_web::{web, App, HttpServer, HttpResponse};
use dotenvy::dotenv;
use std::env;

mod api;
mod db;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let pool = db::establish_pool(&database_url);

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    log::info!("Starting Insurance Twin API at http://{}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(|| async { HttpResponse::Ok().json(serde_json::json!({"status": "ok"})) }))
            .configure(api::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
