use carmine_api_core::network::{Network, LEGACY_AMM_CONTRACT_ADDRESS};
use carmine_api_db::get_events_by_address;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let legacy_events = get_events_by_address(&Network::Mainnet, LEGACY_AMM_CONTRACT_ADDRESS);

    println!("event count: {}", legacy_events.len());
}
