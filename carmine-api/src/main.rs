mod handlers;
mod types;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use carmine_api_db::models::NewEvent;
use carmine_api_db::{self, get_option_addresses_from_options};
use carmine_api_starknet::Carmine;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::time::{sleep, Duration};
use types::AppState;

const REFETCH_DELAY_SECONDS: u64 = 300;

async fn update_options(carmine: &Carmine) {
    let options = carmine.get_options_with_addresses().await;
    carmine_api_db::create_batch_of_options(&options);
}

async fn update_events() {
    let new_events = carmine_api_starknet::get_new_events_from_starkscan().await;
    let available_option_addresses = get_option_addresses_from_options();
    // filter out events with options not stored in the DB
    let valid_events: Vec<NewEvent> = new_events
        .into_iter()
        .filter(|event| available_option_addresses.contains(&event.option_address))
        .collect();
    carmine_api_db::create_batch_of_events(&valid_events);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let carmine = carmine_api_starknet::Carmine::new();

    let app_data = Data::new(Arc::new(Mutex::new(AppState {
        all_non_expired: carmine.get_all_non_expired_options_with_premia().await,
        trade_history: carmine_api_db::get_trade_history(),
    })));

    let app_data1 = app_data.clone();

    actix_web::rt::spawn(async move {
        loop {
            sleep(Duration::from_secs(REFETCH_DELAY_SECONDS)).await;
            let start = Instant::now();
            println!("Updating AppState");
            update_options(&carmine).await;
            println!("Options updated in {}s", start.elapsed().as_secs());
            update_events().await;
            println!("Events updated in {}s", start.elapsed().as_secs());
            let all_non_expired = carmine.get_all_non_expired_options_with_premia().await;
            let trade_history = carmine_api_db::get_trade_history();
            let mut app_data = app_data1.lock().unwrap();
            *app_data = AppState {
                all_non_expired,
                trade_history,
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
