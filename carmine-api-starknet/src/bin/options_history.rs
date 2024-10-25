use carmine_api_core::{network::Network, types::DbBlock};
use carmine_api_db::get_non_expired_options;
use carmine_api_starknet::carmine::filter_deployed_options;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    const TWO_DAYS_SECS: i64 = 172800;
    let block = DbBlock {
        block_number: 654321,
        timestamp: 1719942864,
    };

    let non_expired_options =
        get_non_expired_options(&Network::Mainnet, block.timestamp - TWO_DAYS_SECS);

    println!("ALL {}", &non_expired_options.len(),);

    let deployed_in_this_block =
        filter_deployed_options(non_expired_options, block.block_number).await;

    println!("DEPLOYED {}", &deployed_in_this_block.len(),);
}
