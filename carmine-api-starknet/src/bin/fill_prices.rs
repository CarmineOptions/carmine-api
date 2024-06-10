use carmine_api_core::{
    network::Network,
    types::{DbBlock, OracleName, TokenPair},
};
use carmine_api_db::{create_oracle_price, get_price_block_numbers};
use carmine_api_starknet::oracle::Oracle;
use dotenvy::dotenv;
use std::collections::HashSet;
use tokio::task;

async fn add_price_for_block(
    pragma: &Oracle,
    token_pair: &TokenPair,
    block: &DbBlock,
) -> Result<(), String> {
    let pragma_price_result = pragma.get_spot_median(token_pair, block).await;

    let pragma_price = match pragma_price_result {
        Ok(price) => price,
        Err(e) => return Err(e),
    };

    create_oracle_price(&pragma_price, &Network::Mainnet);
    println!("updated prices for block {}", block.block_number);
    Ok(())
}

pub fn get_missing_block_numbers(
    token_pair: &TokenPair,
    min_block: i64,
    max_block: i64,
) -> Vec<i64> {
    let existing_block_numbers = get_price_block_numbers(token_pair, min_block, max_block);
    let existing_block_numbers_set: HashSet<_> = existing_block_numbers.into_iter().collect();
    let all_block_numbers: Vec<i64> = (min_block..=max_block).collect();
    let mut missing_block_numbers: Vec<i64> = all_block_numbers
        .into_iter()
        .filter(|block_number| !existing_block_numbers_set.contains(block_number))
        .collect();

    missing_block_numbers.sort();
    missing_block_numbers
}

async fn fill_prices(token_pair: &TokenPair, start_block: i64, end_block: i64) {
    let blocks = get_missing_block_numbers(token_pair, start_block, end_block);
    println!(
        "Updating {} blocks between {} - {}",
        blocks.len(),
        start_block,
        end_block
    );
    let pragma = Oracle::new(OracleName::Pragma);

    for block in blocks {
        let b = &DbBlock {
            block_number: block,
            timestamp: 0,
        };

        let res = add_price_for_block(&pragma, token_pair, b).await;

        match res {
            Ok(_) => println!("{} - {} Ok", token_pair.id(), block),
            Err(e) => println!("{} - {} Failed! {}", token_pair.id(), block, e),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 557529 is first block with STRK price

    let end = 630000;

    let task1 = task::spawn(fill_prices(&TokenPair::BtcUsdc, 589419, end));
    let task2 = task::spawn(fill_prices(&TokenPair::EthUsdc, 589419, end));
    let task3 = task::spawn(fill_prices(&TokenPair::StrkUsdc, 557529, end));

    let _ = tokio::join!(task1, task2, task3);
}
