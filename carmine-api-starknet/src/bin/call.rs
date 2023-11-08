use carmine_api_rpc_gateway::{
    blast_api_latest_block_number, carmine_latest_block_number, infura_latest_block_number,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c = carmine_latest_block_number().await.unwrap();
    let b = blast_api_latest_block_number().await.unwrap();
    let i = infura_latest_block_number().await.unwrap();

    println!("carmine: {}, blast api: {}, infura: {}", c, b, i);
}
