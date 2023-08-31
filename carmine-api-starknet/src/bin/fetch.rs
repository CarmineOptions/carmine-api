use std::env;

use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;
use carmine_api_starknet::starkscan::get_block_range_events;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let _zklend_genesis_block = 48660;
    let _hashstack_genesis_block = 21178;

    let first_block = 179000;
    let last_block = 183930;
    let mut cur_from = first_block;
    let increment = 500;

    while cur_from < last_block {
        let events = get_block_range_events(
            &Protocol::NostraInterestModel,
            cur_from,
            cur_from + increment,
        )
        .await;
        create_batch_of_starkscan_events(&events, &Network::Mainnet);
        println!("Updated {} - {}", cur_from, cur_from + increment);
        cur_from += increment;
    }

    println!("DONE")
}
