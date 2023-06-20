use std::env;

use carmine_api_core::network::Protocol;
use carmine_api_starknet::starkscan::update_lending_protocol_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.76.28.66");

    update_lending_protocol_events(&Protocol::Hashstack).await;
    update_lending_protocol_events(&Protocol::ZkLend).await;
}
