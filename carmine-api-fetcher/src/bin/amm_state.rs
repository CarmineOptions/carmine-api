use carmine_api_starknet::update_database_amm_state;

use dotenvy::dotenv;

const BLOCK_OFFSET: i64 = 5;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let res = update_database_amm_state(BLOCK_OFFSET).await;

    println!("Updated database amm state {:?}", res);
}
