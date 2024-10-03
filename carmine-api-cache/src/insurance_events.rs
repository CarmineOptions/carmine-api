use std::time::UNIX_EPOCH;

use carmine_api_core::{
    types::{InsuranceData, InsuranceEventQueryable},
    utils::{math_64_to_decimal, strike_from_hex},
};
use carmine_api_prices::HistoricalPrices;

pub fn compose_insurance_event(
    event: &InsuranceEventQueryable,
    prices: &HistoricalPrices,
) -> InsuranceData {
    let base_token_address = event
        .calldata
        .get(7)
        .expect("failed getting base token for insurance event");
    let premia = event
        .calldata
        .get(8)
        .expect("failed getting premia for insurance event");
    let strike = event
        .calldata
        .get(1)
        .expect("failed getting strike for insurance event");
    let size = event
        .calldata
        .get(5)
        .expect("failed getting premia for insurance event");
    let pool_id = match base_token_address.as_str() {
        "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7" => "eth-usdc-call",
        "0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac" => "btc-usdc-call",
        "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d" => "strk-usdc-call",
        _ => unreachable!("invalid base token"),
    };
    let timestamp = event
        .timestamp
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    let price = prices.get_price(&pool_id, carmine_api_prices::BlockId::Timestamp(timestamp));

    InsuranceData {
        user_address: event.user_address.to_string(),
        base_token_price: price,
        timestamp,
        base_token_address: base_token_address.to_string(),
        premia: math_64_to_decimal(&premia),
        strike: strike_from_hex(&strike),
        size: size.to_string(),
    }
}

pub fn get_insurace_data(
    events: Vec<InsuranceEventQueryable>,
    prices: &HistoricalPrices,
) -> Vec<InsuranceData> {
    events
        .iter()
        .map(|e| compose_insurance_event(e, prices))
        .collect()
}
