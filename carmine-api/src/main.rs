mod handlers;
mod types;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use carmine_api_cache::Cache;
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use types::AppState;

const REFETCH_DELAY_SECONDS: u64 = 600;
const LOCAL_IP: &str = "127.0.0.1";
const DOCKER_IP: &str = "0.0.0.0";

struct Origins {}

impl Origins {
    const LOCAL: &str = "http://localhost:3000";
    const DEVELOPMENT: &str = "https://app.carmine-dev.eu";
    const PRODUCTION: &str = "https://app.carmine.finance";
}

fn ip_address() -> &'static str {
    let is_local_build = match env::var("ENVIRONMENT") {
        Ok(v) => v == "local",
        _ => false,
    };

    if is_local_build {
        return LOCAL_IP;
    }
    DOCKER_IP
}

/// Checks necessary ENV variables and panics if any is missing
fn startup_check() {
    env::var("NETWORK").expect("Check failed - env var \"NETWORK\" is not set");
    env::var("ENVIRONMENT").expect("Check failed - env var \"ENVIRONMENT\" is not set");
    env::var("STARKSCAN_API_KEY").expect("Check failed - env var \"STARKSCAN_API_KEY\" is not set");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    startup_check();
    println!("👷 Starting server");

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let app_data = Data::new(Arc::new(Mutex::new(AppState {
        all_non_expired: Vec::new(),
        trade_history: Vec::new(),
        ready: false,
    })));

    let app_data1 = app_data.clone();

    actix_web::rt::spawn(async move {
        let mut should_update = false;
        let mut cache = Cache::new().await;

        loop {
            // do not update fresh cache, then update everytime
            if should_update {
                sleep(Duration::from_secs(REFETCH_DELAY_SECONDS)).await;
                println!("Updating AppState");
                cache.update().await;
            } else {
                should_update = true;
            }
            let all_non_expired = cache.get_all_non_expired();
            let trade_history = cache.get_trade_history();
            let mut app_data = app_data1.lock().unwrap();
            *app_data = AppState {
                all_non_expired,
                trade_history,
                ready: true,
            };
            println!("AppState updated");
        }
    });

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(Origins::LOCAL)
            .allowed_origin(Origins::DEVELOPMENT)
            .allowed_origin(Origins::PRODUCTION)
            .allowed_methods(vec!["GET"])
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
    // .bind(("127.0.0.1", 8000))?
    .bind((ip_address(), 8000))?
    .run()
    .await
}
