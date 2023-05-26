use std::env;

use carmine_api_core::network::Network;
use carmine_api_db::{get_block_by_number, update_option_volatility};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.76.28.66");

    let network = Network::Mainnet;
    let carmine = Carmine::new(Network::Mainnet);

    let mut block_number = 45237;

    loop {
        if let Some(block) = get_block_by_number(block_number, &network) {
            if let Ok(res) = carmine.get_all_options_volatility(&block).await {
                for data in res {
                    update_option_volatility(
                        &network,
                        block_number,
                        data.volatility,
                        data.option_position,
                        data.option_address,
                    );
                }
                println!("Updated block #{}", block_number);
                block_number -= 1;
            }
        } else {
            println!("Failed to get block {}", block_number);
            break;
        }
    }
}
