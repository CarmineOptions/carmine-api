use std::env;

use carmine_api_core::network::Protocol;
use carmine_api_starknet::starkscan::update_block_range;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.76.28.66");

    update_block_range(&Protocol::ZkLend, 48600, 48700).await;
}
