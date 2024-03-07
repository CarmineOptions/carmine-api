use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let so = AmmStateObserver::new();

    let _res = so.update_single_block(600000).await;
}
