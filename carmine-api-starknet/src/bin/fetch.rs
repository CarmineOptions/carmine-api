use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;

use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = &Network::Mainnet;
    let protocols = vec![&Protocol::Nostra2USDTCollateral];

    let start = 168000;
    let mut current;
    let increment = 1000;
    let max = 473000;

    for protocol in protocols {
        current = start;

        while current < max {
            let events =
                get_block_range_events(protocol, network, current, current + increment).await;
            println!("{} fetched {} - {}", protocol, current, current + increment);
            current = current + increment;
            create_batch_of_starkscan_events(&events, network);
        }
    }
}
