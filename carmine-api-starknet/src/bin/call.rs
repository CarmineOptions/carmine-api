use std::env;

use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let state_observer = AmmStateObserver::new();
    state_observer
        .update_state_over_block_range(190500, 191000, 1)
        .await;
}
