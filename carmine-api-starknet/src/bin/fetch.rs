use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;

use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut current = 348095;
    let increment = 5;
    let max = 380000;

    while current < max {
        let events = get_block_range_events(
            &Protocol::ZkLend,
            &Network::Mainnet,
            current,
            current + increment,
        )
        .await;
        create_batch_of_starkscan_events(&events, &Network::Mainnet);
        println!("fetched {} - {}", current, current + increment);
        current = current + increment;
    }
}
