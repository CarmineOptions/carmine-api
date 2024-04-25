use std::collections::HashMap;

use carmine_api_core::{
    types::{DefispringInfo, PriceResponse},
    utils::get_coingecko_prices,
};
use carmine_api_db::get_pool_tvl_map;

async fn get_tvl_in_usd(prices: PriceResponse) -> f64 {
    let mut pool_tvl_map = get_pool_tvl_map();

    // remove BTC pools for DefiSpring
    pool_tvl_map.remove("0x35db72a814c9b30301f646a8fa8c192ff63a0dc82beb390a36e6e9eba55b6db"); // BTC USDC CALL
    pool_tvl_map.remove("0x1bf27366077765c922f342c8de257591d1119ebbcbae7a6c4ff2f50ede4c54c"); // BTC USDC PUT

    let pool_to_price_map = HashMap::from([
        (
            "0x2b629088a1d30019ef18b893cebab236f84a365402fa0df2f51ec6a01506b1d",
            prices.starknet.usd,
        ),
        (
            "0x4dcd9632353ed56e47be78f66a55a04e2c1303ebcb8ec7ea4c53f4fdf3834ec",
            prices.starknet.usd,
        ),
        (
            "0x6df66db6a4b321869b3d1808fc702713b6cbb69541d583d4b38e7b1406c09aa",
            prices.ethereum.usd,
        ),
        (
            "0x70cad6be2c3fc48c745e4a4b70ef578d9c79b46ffac4cd93ec7b61f951c7c5c",
            prices.ethereum.usd,
        ),
        (
            "0x466e3a6731571cf5d74c5b0d9c508bfb71438de10f9a13269177b01d6f07159",
            prices.usd_coin.usd,
        ),
        (
            "0x6ebf1d8bd43b9b4c5d90fb337c5c0647b406c6c0045da02e6675c43710a326f",
            prices.usd_coin.usd,
        ),
    ]);

    let mut tvl = 0.0;

    let ad_hoc_precission = 1000.0;

    for (key, value) in pool_tvl_map.into_iter() {
        let price = pool_to_price_map
            .get(&key.as_str())
            .expect("Could not find price");
        let result_f64 = value as f64 * price / ad_hoc_precission;
        tvl += result_f64;
    }

    tvl
}

// TODO: implement
async fn get_starknet_incentive() -> f64 {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    145.0
}

pub async fn get_defispring_stats() -> DefispringInfo {
    let prices: PriceResponse = get_coingecko_prices().await.expect("Failed getting prices");
    let strk_in_usd = prices.starknet.usd;
    let tvl_usd = get_tvl_in_usd(prices).await;
    let strk_incentive = get_starknet_incentive().await;

    let apy = (strk_incentive * strk_in_usd * 365.0 / tvl_usd) * 100.0;

    DefispringInfo {
        tvl: tvl_usd,
        strk_incentive,
        apy,
    }
}
