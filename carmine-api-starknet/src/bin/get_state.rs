use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let amm_state_observer = AmmStateObserver::new();

    let state_res = amm_state_observer.update_single_block(390010).await;

    println!("{:?}", state_res);
}
