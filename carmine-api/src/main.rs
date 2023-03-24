mod handlers;
mod types;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use carmine_api_cache::Cache;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::time::{sleep, Duration};
use types::AppState;

const REFETCH_DELAY_SECONDS: u64 = 300;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let cache = Cache::new().await;

    let app_data = Data::new(Arc::new(Mutex::new(AppState {
        all_non_expired: cache.all_non_expired,
        trade_history: cache.trade_history,
    })));

    let app_data1 = app_data.clone();

    actix_web::rt::spawn(async move {
        loop {
            sleep(Duration::from_secs(REFETCH_DELAY_SECONDS)).await;
            let start = Instant::now();
            println!("Updating AppState");
            let cache2 = Cache::new().await;
            println!("Cache updated in {}s", start.elapsed().as_secs());
            let mut app_data = app_data1.lock().unwrap();
            *app_data = AppState {
                all_non_expired: cache2.all_non_expired,
                trade_history: cache2.trade_history,
            };
            println!("AppState updated in {}s", start.elapsed().as_secs());
        }
    });

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(app_data.clone())
            .configure(handlers::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
