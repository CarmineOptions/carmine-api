use carmine_api_core::{
    network::Network,
    pool::{
        MAINNET_BTC_USDC_CALL, MAINNET_BTC_USDC_PUT, MAINNET_ETH_STRK_CALL, MAINNET_ETH_STRK_PUT,
        MAINNET_ETH_USDC_CALL, MAINNET_ETH_USDC_PUT, MAINNET_STRK_USDC_CALL, MAINNET_STRK_USDC_PUT,
    },
    types::{OraclePrice, PoolStatePriceUpdate},
    utils::string_to_float,
};
use carmine_api_db::{get_pool_states_with_prices, update_pool_state_asset_prices};
use dotenvy::dotenv;

fn oracle_to_price(oracle: &OraclePrice) -> f64 {
    let decimals: i32 = oracle.decimals as i32;
    let raw_price: f64 = oracle.price as f64;

    raw_price / 10f64.powi(decimals)
}

fn main() {
    dotenv().ok();
    let net = &Network::Mainnet;

    let address_pair = vec![
        (MAINNET_ETH_USDC_CALL.address, "eth-usdc"),
        (MAINNET_ETH_USDC_PUT.address, "usdc"),
        (MAINNET_BTC_USDC_CALL.address, "btc-usdc"),
        (MAINNET_BTC_USDC_PUT.address, "usdc"),
        (MAINNET_ETH_STRK_CALL.address, "eth-usdc"),
        (MAINNET_ETH_STRK_PUT.address, "strk-usdc"),
        (MAINNET_STRK_USDC_CALL.address, "strk-usdc"),
        (MAINNET_STRK_USDC_PUT.address, "usdc"),
    ];

    for (pool_address, token_pair) in address_pair {
        let pool_states = get_pool_states_with_prices(pool_address, net);

        let mut updates = vec![];

        println!("Iterating over {} items", pool_states.len());

        for rich_pool_state in pool_states {
            let (pool_state, prices) = rich_pool_state;

            let price = match token_pair {
                "usdc" => 1.0,
                pair => {
                    let price_struct_option: Option<&OraclePrice> =
                        prices.iter().find(|p| p.token_pair == pair);
                    if price_struct_option.is_none() {
                        continue;
                    }
                    oracle_to_price(price_struct_option.unwrap())
                }
            };

            if let Some(lp_token_value) = pool_state.lp_token_value {
                let lp_float = string_to_float(lp_token_value.as_str(), 18);

                let info = PoolStatePriceUpdate {
                    lp_token_value_usd: lp_float * price,
                    underlying_asset_price: price,
                    block_number: pool_state.block_number,
                    lp_address: pool_state.lp_address,
                };

                updates.push(info);
            }
        }

        println!("Got {} pool updates", updates.len());

        match update_pool_state_asset_prices(updates) {
            Ok(_) => println!("{} Succeeded", pool_address),
            Err(e) => println!("{} Failed: {:#?}", pool_address, e),
        }
    }
}
