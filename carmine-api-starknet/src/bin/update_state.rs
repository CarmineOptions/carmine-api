use carmine_api_core::network::Network;
use carmine_api_db::{get_blocks_greater_than, update_batch_of_volatilities};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mainnet = Network::Mainnet;

    let carmine = Carmine::new(mainnet);
    let blocks = get_blocks_greater_than(504260, &mainnet);

    for block in blocks {
        println!("BLOCK {}", block.block_number);
        let volatilities_res = carmine.get_all_options_volatility(&block).await;
        if let Ok(volatilities) = volatilities_res {
            update_batch_of_volatilities(&volatilities, &mainnet);
        }
    }
}
