use carmine_api_core::{network::Network, pool::MAINNET_STRK_USDC_CALL};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c = Carmine::new(Network::Mainnet);

    let block_number = 618396;
    let pool_address = MAINNET_STRK_USDC_CALL.address;

    let locked = c
        .get_pool_locked_capital(block_number, pool_address.to_string())
        .await;
    let unlocked = c
        .get_unlocked_capital(block_number, pool_address.to_string())
        .await;

    println!("{:#?}", locked);
    println!("{:#?}", unlocked);
}
