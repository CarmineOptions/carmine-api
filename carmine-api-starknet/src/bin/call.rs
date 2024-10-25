use carmine_api_core::network::Network;
use carmine_api_db::get_pool_state_block_holes;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let start = 504056; // new AMM deployed
    let finish = 827949;

    let holes = get_pool_state_block_holes(start, finish, &Network::Mainnet);

    println!("{:?}", holes);
}
