use std::env;

use carmine_api_core::network::Protocol;
use carmine_api_starknet::starkscan::update_block_range;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.76.28.66");

    let zklend_genesis_block = 48660;

    let first_block = zklend_genesis_block;
    let last_block = 97000;
    let mut cur_from = first_block;
    let increment = 1000;

    loop {
        update_block_range(&Protocol::ZDAI, cur_from, cur_from + increment).await;
        cur_from += increment;
        if cur_from > last_block {
            break;
        }
    }

    println!("DONE")
}
