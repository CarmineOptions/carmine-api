mod handlers;
mod telegram_bot;
mod types;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use carmine_api_airdrop::merkle_tree::MerkleTree;
use carmine_api_cache::Cache;
use carmine_api_core::network::Network;
use carmine_api_core::types::AppState;
use carmine_api_starknet::{update_database_amm_state, update_database_events};
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

const UPDATE_APP_STATE_INTERVAL: u64 = 600;
const UPDATE_EVENTS_INTERVAL: u64 = 900;
const UPDATE_AMM_STATE_INTERVAL: u64 = 1200;

const LOCAL_IP: &str = "127.0.0.1";
const DOCKER_IP: &str = "0.0.0.0";

struct Origins {}

impl Origins {
    const LOCAL: &str = "http://localhost:3000";
    const DEVELOPMENT: &str = "https://app.carmine-dev.eu";
    const PRODUCTION: &str = "https://app.carmine.finance";
}

fn ip_address() -> &'static str {
    match env::var("ENVIRONMENT") {
        Ok(v) if v == "local" => LOCAL_IP,
        _ => DOCKER_IP,
    }
}

/// Checks necessary ENV variables and panics if any is missing
fn startup_check() {
    let environment = env::var("ENVIRONMENT").expect("ENV \"ENVIRONMENT\" is not set");
    env::var("STARKSCAN_API_KEY").expect("ENV \"STARKSCAN_API_KEY\" is not set");
    if environment.as_str() != "local" {
        // only check those if not connecting to local DB
        env::var("DB_USER").expect("ENV \"DB_USER\" is not set");
        env::var("DB_PASSWORD").expect("ENV \"DB_PASSWORD\" is not set");
        env::var("DB_IP").expect("ENV \"DB_IP\" is not set");
    }
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

    println!("🛠️  Creating cache instances...");

    let mut mainnet_cache = Cache::new(Network::Mainnet);
    let mut testnet_cache = Cache::new(Network::Testnet);

    println!("🛠️  Getting data from DB...");

    let mainnet = mainnet_cache.get_app_data();

    println!("✨ Got Mainnet data");

    let testnet = testnet_cache.get_app_data();

    println!("✨ Got Testnet data");

    println!("✨ Got Airdrop data");

    println!("🛠️  Creating app state...");

    let airdrop = MerkleTree::new();

    let app_state = Data::new(Arc::new(Mutex::new(AppState {
        mainnet,
        testnet,
        airdrop,
    })));

    println!("🛠️  Cloning app state...");

    let app_state1 = app_state.clone();

    println!("🛠️  Spawning app state updating thread...");

    // updates app state
    actix_web::rt::spawn(async move {
        let mut startup = true;
        loop {
            if startup {
                startup = false;
            } else {
                sleep(Duration::from_secs(UPDATE_APP_STATE_INTERVAL)).await;
            }
            println!("Updating AppState");
            mainnet_cache.update().await;
            testnet_cache.update().await;
            let airdrop = MerkleTree::new();

            let mut app_state = app_state1.lock().unwrap();
            *app_state = AppState {
                mainnet: mainnet_cache.get_app_data(),
                testnet: testnet_cache.get_app_data(),
                airdrop,
            };
            println!("AppState updated");
        }
    });

    println!("🛠️  Spawning DB events updating thread...");
    // fetches events and updates database
    // events fetching is fast and there is a limit
    // on requests we can make, therefore it is important
    // to wait in between executions to avoid "Limit exceeded"
    actix_web::rt::spawn(async {
        let mut startup: bool = true;
        let mut should_report = true;
        loop {
            if startup {
                startup = false;
            } else {
                sleep(Duration::from_secs(UPDATE_EVENTS_INTERVAL)).await;
            }
            if let Err(err) = actix_web::rt::spawn(async { update_database_events().await }).await {
                if should_report {
                    println!("Update database events panicked\n\n{:?}", err);
                    telegram_bot::send_message(
                        "Carmine API `update_database_events` just panicked",
                    )
                    .await;
                    // prevent multiple messages for the same problem
                    should_report = false;
                }
            } else {
                println!("Database updated with events");
            }
        }
    });

    println!("🛠️  Spawning DB amm state updating thread...");
    // fetches amm state and updates database
    // blockchain calls are slow and there is
    // no limit on how many can be made, therefore
    // no sleep is required in this loop
    actix_web::rt::spawn(async {
        let mut startup: bool = true;
        let mut should_report = true;
        loop {
            if startup {
                startup = false;
            } else {
                sleep(Duration::from_secs(UPDATE_AMM_STATE_INTERVAL)).await;
            }
            if let Err(err) =
                actix_web::rt::spawn(async { update_database_amm_state().await }).await
            {
                if should_report {
                    println!("Update database amm state panicked\n\n{:?}", err);
                    telegram_bot::send_message(
                        "Carmine API `update_database_amm_state` just panicked",
                    )
                    .await;
                    // prevent multiple messages for the same problem
                    should_report = false;
                }
            } else {
                println!("Database updated with AMM state");
            }
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
            .app_data(app_state.clone())
            .configure(handlers::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    // .bind(("127.0.0.1", 8000))?
    .bind((ip_address(), 8000))?
    .run()
    .await
}
