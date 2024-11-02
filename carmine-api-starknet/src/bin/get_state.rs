use carmine_api_core::{network::Network, types::DbBlock};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let b = DbBlock {
        block_number: 864166,
        timestamp: 1730500755,
    };
    let c = Carmine::new(Network::Mainnet);
    let state = c.get_amm_state(&b).await.unwrap();

    println!("State: {:#?}", state);
}
