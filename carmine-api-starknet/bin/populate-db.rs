use carmine_api_core::network::Network;
use carmine_api_db::{get_events, get_options};
use carmine_api_starknet::{carmine::Carmine, starkscan::get_events_from_starkscan};
use dotenvy::dotenv;

async fn populate_network(n: &Network) {
    let c = Carmine::new(*n);
    get_events_from_starkscan(n).await;
    c.get_options_with_addresses().await;
}

fn validate_db(n: &Network) {
    println!(
        "Network {}\nevents: {}\noptions: {}",
        n,
        get_events(n).len(),
        get_options(n).len(),
    );
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let networks = vec![Network::Testnet, Network::Mainnet];

    for n in networks.iter() {
        populate_network(&n).await;
        validate_db(&n);
    }
}
