use carmine_api_rpc_gateway::{
    blast_api_latest_block_number, carmine_latest_block_number, infura_latest_block_number,
};
use dotenvy::dotenv;
use tokio::try_join;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let latest_blocks = try_join!(
        carmine_latest_block_number(),
        blast_api_latest_block_number(),
    );

    println!("{:#?}", latest_blocks);
}
