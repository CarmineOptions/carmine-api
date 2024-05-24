use std::fs::File;

use carmine_api_core::{
    network::Network,
    pool::MAINNET_ETH_USDC_CALL,
    types::{OraclePrice, PoolState},
    utils::string_to_float,
};
use carmine_api_db::get_pool_states_with_prices;
use dotenvy::dotenv;
use serde_json::to_writer_pretty;

fn oracle_to_price(oracle: &OraclePrice) -> f64 {
    let decimals: i32 = oracle.decimals as i32;
    let raw_price: f64 = oracle.price as f64;

    raw_price / 10f64.powi(decimals)
}

fn main() {
    dotenv().ok();
    let net = &Network::Mainnet;
    let pool_states = get_pool_states_with_prices(MAINNET_ETH_USDC_CALL.address, net);

    let eth_token_pair = "eth-usdc";

    let mut infos = vec![];

    println!("Iterating over {} items", pool_states.len());

    for rich_pool_state in pool_states {
        let (pool_state, prices) = rich_pool_state;
        let eth_price_struct_options: Option<&OraclePrice> =
            prices.iter().find(|p| p.token_pair == eth_token_pair);
        if eth_price_struct_options.is_none() {
            continue;
        }
        if let Some(lp_token_value) = pool_state.lp_token_value {
            let lp_float = string_to_float(lp_token_value.as_str(), 18);
            let eth_price = oracle_to_price(eth_price_struct_options.unwrap());

            let info = PoolState {
                unlocked_cap: pool_state.unlocked_cap,
                locked_cap: pool_state.locked_cap,
                lp_balance: pool_state.lp_balance,
                pool_position: pool_state.pool_position,
                lp_token_value: Some(lp_token_value),
                lp_token_value_usd: Some(lp_float * eth_price),
                underlying_asset_price: Some(eth_price),
                block_number: pool_state.block_number,
                lp_address: pool_state.lp_address,
            };

            infos.push(info);
        }
    }

    println!("Got {} pool infos", infos.len());

    let file = File::create("./pool_infos.json").expect("Failed creating file");
    to_writer_pretty(file, &infos).expect("Failed writing to file");
}
