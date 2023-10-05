use carmine_api_starknet::update_database_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Updating events...");
    update_database_events().await;
    println!("DONE");
}
