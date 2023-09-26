use std::env;

use carmine_api_core::{network::Network, telegram_bot};
use carmine_api_rpc_gateway::{blast_api_latest_block_number, carmine_latest_block_number};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let c = Carmine::new(Network::Testnet);
    let res = c.get_all_non_expired_options_with_premia().await;

    println!("{:#?}", res);
}
