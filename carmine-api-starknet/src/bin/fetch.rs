use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;

use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut current = 284000;
    let increment = 500;

    while current < 285500 {
        let events = get_block_range_events(&Protocol::ZkLend, current, current + increment).await;
        create_batch_of_starkscan_events(&events, &Network::Mainnet);
        println!("fetched {} - {}", current, current + increment);
        current = current + increment;
    }

    let events = get_block_range_events(&Protocol::ZkLend, 277000, 277000).await;

    println!("fetched {} events", &events.len());

    create_batch_of_starkscan_events(&events, &Network::Mainnet);
}
