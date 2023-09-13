use std::env;

use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let bad_blocks: Vec<i64> = vec![];

    let observer = AmmStateObserver::new();

    for block_number in bad_blocks {
        match observer.update_single_block(block_number).await {
            Ok(_) => {
                println!("{} OK", block_number);
            }
            Err(_) => {
                println!("FAILED {}", block_number);
            }
        }
    }

    println!("DONE");
}
