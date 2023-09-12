use std::env;

use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let state_observer = AmmStateObserver::new();
    let _ = state_observer.update_single_block(200000).await;
}
