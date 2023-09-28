use std::env;

use carmine_api_core::network::Network;

use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let carmine = Carmine::new(Network::Testnet);
    carmine.get_options_with_addresses().await;

    println!("Updated Carmine Testnet events");
}
