use std::env;

use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;

use carmine_api_starknet::starkscan::get_protocol_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let testnet_carmine_events =
        get_protocol_events(&Network::Testnet, &Protocol::CarmineOptions).await;

    create_batch_of_starkscan_events(&testnet_carmine_events, &Network::Testnet);

    println!("Updated Carmine Testnet events");
}
