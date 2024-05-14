use std::collections::HashSet;

use carmine_api_core::network::{Network, Protocol};

use carmine_api_db::create_batch_of_starkscan_events;
use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = &Network::Mainnet;
    let protocols = vec![
        &Protocol::CarminePoolEthUsdcCall,
        &Protocol::CarminePoolEthUsdcPut,
        &Protocol::CarminePoolBtcUsdcCall,
        &Protocol::CarminePoolBtcUsdcPut,
        &Protocol::CarminePoolEthStrkCall,
        &Protocol::CarminePoolEthStrkPut,
        &Protocol::CarminePoolStrkUsdcCall,
        &Protocol::CarminePoolStrkUsdcPut,
    ];

    let start = 504000;
    let mut current;
    let increment = 4000;
    let max = 640000;

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
    let mut unique = HashSet::new();
    let mut result = vec![];

    for item in events.into_iter() {
        let key = format!("{}-{}", item.transaction_hash.clone(), item.event_index);
        if unique.insert(key) {
            // The hash was successfully inserted, meaning it was not present before.
            result.push(item);
        }
    }

    create_batch_of_starkscan_events(&result, network);
}
