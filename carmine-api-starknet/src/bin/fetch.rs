use carmine_api_core::network::{Network, Protocol};

use carmine_api_db::create_batch_of_starkscan_events;
use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = &Network::Mainnet;
    let protocols = vec![&Protocol::Pail];

    let start = 894431;
    let mut current;
    let increment = 21;
    let max = 894449;

    let mut events = vec![];

    for protocol in protocols {
        current = start;

        while current < max {
            let mut new_events =
                get_block_range_events(protocol, network, current - 1, current + increment + 1)
                    .await;
            println!("{} fetched {} - {}", protocol, current, current + increment);
            current = current + increment;
            events.append(&mut new_events);
        }
    }

    create_batch_of_starkscan_events(&events, network);
}
