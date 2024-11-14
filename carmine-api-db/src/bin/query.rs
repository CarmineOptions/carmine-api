use carmine_api_core::{network::Network, pool::MAINNET_ETH_USDC_CALL};
use carmine_api_db;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let state = carmine_api_db::get_pool_state(&MAINNET_ETH_USDC_CALL.address, &Network::Mainnet);

    let max_element = state.iter().max_by_key(|v| v.block_number);

    println!("{:#?}", max_element);
}
