use std::env;

use carmine_api_starknet::update_database_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    println!("Updating events...");
    update_database_events().await;
    println!("DONE");
}