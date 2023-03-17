mod handlers;
mod response;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use carmine_api_starknet;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

pub struct AppState {
    pub all_non_expired: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

const REFETCH_DELAY_SECONDS: u64 = 300;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let carmine = carmine_api_starknet::Carmine::new();

    let app_data = Data::new(Arc::new(Mutex::new(AppState {
        all_non_expired: carmine.get_all_non_expired_options_with_premia().await,
    })));

    let app_data1 = app_data.clone();

    actix_web::rt::spawn(async move {
        loop {
            sleep(Duration::from_secs(REFETCH_DELAY_SECONDS)).await;
            println!("Refetching the data");
            let fetched_all_non_expired = carmine.get_all_non_expired_options_with_premia().await;
            let mut app_data = app_data1.lock().unwrap();
            *app_data = AppState {
                all_non_expired: fetched_all_non_expired,
            };
            println!("Data updated");
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
