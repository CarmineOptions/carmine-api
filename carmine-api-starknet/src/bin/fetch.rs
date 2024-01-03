use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;

use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = &Network::Mainnet;
    let protocols = vec![&Protocol::Hashstack2];

    let start = 468500;
    let mut current;
    let increment = 500;
    let max = 492929;

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
