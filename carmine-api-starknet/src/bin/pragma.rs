use carmine_api_core::{
    network::Network,
    types::{OracleName, TokenPair},
};
use carmine_api_db::{create_oracle_price, get_blocks_greater_than};
use carmine_api_starknet::oracle::Oracle;
use dotenvy::dotenv;

fn get_first_block_for_token_pair(token_pair: &TokenPair) -> i64 {
    match token_pair {
        // TokenPair::EthUsdc => 416490, <- first on new Pragma
        TokenPair::EthUsdc => 535150,
        TokenPair::BtcUsdc => 416490,
        TokenPair::StrkUsdc => 557529,
    }
}

async fn update_token_pair(token_pair: TokenPair) {
    let pragma = Oracle::new(OracleName::Pragma);
    let network = Network::Mainnet;
    let first_block_number = get_first_block_for_token_pair(&token_pair);

    println!("Starting from {}", first_block_number);

    let blocks = get_blocks_greater_than(first_block_number, &network);

    for block in blocks {
        // if block.block_number % 100 != 0 {
        //     continue;
        // }
        println!("Updating {}", &block.block_number);
        match pragma.get_spot_median(&token_pair, &block).await {
            Ok(data) => create_oracle_price(&data, &network),
            Err(e) => println!(
                "{} {} Failed, error: {:#?}",
                &block.block_number, &token_pair, e
            ),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    update_token_pair(TokenPair::EthUsdc).await;
}
