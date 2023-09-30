use std::env;

use carmine_api_rpc_gateway::{
    blast_api_latest_block_number, carmine_latest_block_number, infura_latest_block_number,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let c = carmine_latest_block_number().await.unwrap();
    let b = blast_api_latest_block_number().await.unwrap();
    let i = infura_latest_block_number().await.unwrap();

    println!("carmine: {}, blast api: {}, infura: {}", c, b, i);
}
