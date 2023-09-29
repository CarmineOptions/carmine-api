mod handlers;
mod types;

use actix_cors::Cors;
use actix_web::middleware::{self, Logger};
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use carmine_api_airdrop::merkle_tree::MerkleTree;
use carmine_api_cache::Cache;
use carmine_api_core::network::Network;
use carmine_api_core::types::AppState;
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

const UPDATE_APP_STATE_INTERVAL: u64 = 300;

const LOCAL_IP: &str = "127.0.0.1";
const DOCKER_IP: &str = "0.0.0.0";

struct Origins {}

impl Origins {
    const LOCAL: &str = "http://localhost:3000";
    const DEVELOPMENT: &str = "https://app.carmine-dev.eu";
    const PRODUCTION: &str = "https://app.carmine.finance";
    const PRODUCTION_MAINNET: &str = "https://mainnet.app.carmine.finance";
    const PRODUCTION_TESTNET: &str = "https://testnet.app.carmine.finance";
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

    let mut mainnet_cache = Cache::new(Network::Mainnet).await;
    let mut testnet_cache = Cache::new(Network::Testnet).await;

    println!("🛠️  Getting data from DB...");

    let mainnet = mainnet_cache.get_app_data();

    println!("✨ Got Mainnet data");

    let testnet = testnet_cache.get_app_data();

    println!("✨ Got Testnet data");

    let airdrop: MerkleTree = MerkleTree::new();

    println!("✨ Got Airdrop data");

    println!("🛠️  Creating app state...");

    let app_state = Data::new(Arc::new(Mutex::new(AppState {
        mainnet,
        testnet,
        airdrop,
    })));

    println!("🛠️  Cloning app state...");

    let app_state_clone = app_state.clone();

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
            let mainnet = mainnet_cache.get_app_data();
            let testnet = testnet_cache.get_app_data();

            let mut app_state_lock = app_state_clone.lock().unwrap();
            app_state_lock.mainnet = mainnet;
            app_state_lock.testnet = testnet;
            drop(app_state_lock);
            println!("AppState updated");
        }
    });

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(Origins::LOCAL)
            .allowed_origin(Origins::DEVELOPMENT)
            .allowed_origin(Origins::PRODUCTION)
            .allowed_origin(Origins::PRODUCTION_MAINNET)
            .allowed_origin(Origins::PRODUCTION_TESTNET)
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
            .wrap(middleware::Compress::default())
    })
    // .bind(("127.0.0.1", 8000))?
    .bind((ip_address(), 8000))?
    .run()
    .await
}
