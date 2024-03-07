use carmine_api_core::{
    network::Network,
    types::{DbBlock, OracleName, TokenPair},
};
use carmine_api_db::{create_oracle_price, get_block_by_number};
use carmine_api_starknet::{amm_state::AmmStateObserver, oracle::Oracle};
use dotenvy::dotenv;
use futures::future::try_join_all;
use std::{env, time::Duration};
use tokio::time::sleep;

async fn add_price_for_block(pragma: &Oracle, block: &DbBlock) -> Result<(), ()> {
    let pragma_eth_usdc_result = pragma.get_spot_median(&TokenPair::EthUsdc, block).await;
    if let Ok(pragma_eth_usdc) = pragma_eth_usdc_result {
        create_oracle_price(&pragma_eth_usdc, &Network::Mainnet);
        println!("updated prices for block {}", block.block_number);
        return Ok(());
    } else {
        return Err(());
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.76.28.66");

    let state_updater = AmmStateObserver::new();
    let pragma = Oracle::new(OracleName::Pragma);

    let mut current_block_number = 39319;
    let increment = 10;

    loop {
        let mut blocks: Vec<DbBlock> = vec![];
        let mut missing_block_numbers: Vec<i64> = vec![];

        for n in (current_block_number - increment + 1)..=current_block_number {
            let block_res = get_block_by_number(n, &Network::Mainnet);
            match block_res {
                Some(block) => blocks.push(block),
                None => missing_block_numbers.push(n),
            }
        }

        // update state one by one
        loop {
            let block_number = match missing_block_numbers.pop() {
                Some(n) => n,
                None => break,
            };
            sleep(Duration::from_secs(5)).await;
            let res = state_updater.update_single_block(block_number).await;
            match res {
                Ok(_) => println!("Updated state for block {}", block_number),
                Err(_) => {
                    println!("Failed updating state for block {} !!!", block_number);
                    // is still missing, add it back to the vec
                    missing_block_numbers.push(block_number);
                }
            };
        }

        // update prices all at once
        let futures = blocks.iter().map(|b| add_price_for_block(&pragma, b));
        match try_join_all(futures).await {
            Ok(_) => {
                println!(
                    "Updated block range {} - {}",
                    current_block_number,
                    current_block_number - increment + 1
                );
                current_block_number -= increment;
            }
            Err(_) => {
                println!(
                    "Block range {} - {} failed, retrying...",
                    current_block_number,
                    current_block_number - increment + 1
                );
            }
        }
    }
}
