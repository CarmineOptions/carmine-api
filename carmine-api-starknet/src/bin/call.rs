use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let amm_state = AmmStateObserver::new();
    loop {
        amm_state.update_state().await;
    }
}
