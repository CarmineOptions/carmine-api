use std::collections::HashMap;

use carmine_api_core::{
    pool::{pool_id_to_base_decimals, pool_id_to_decimals},
    types::{TradeEvent, TradeEventWithPrice, Trades},
    utils::string_to_float,
};
use carmine_api_prices::HistoricalPrices;

fn calculate_premia(
    side: i16,
    _type: i16,
    capital_transfered: f64,
    tokens_minted: f64,
    strike_price: f64,
) -> f64 {
    // Long - premia is the capital transfered
    if side == 0 {
        return capital_transfered;
    }
    // Short Call - premia is the difference between tokens minted and capital transfered
    if _type == 0 {
        return tokens_minted - capital_transfered;
    }
    // Short Put
    tokens_minted * strike_price - capital_transfered
}

fn transform_trade_event(
    event: &TradeEvent,
    prices: &HistoricalPrices,
    pool_id: &str,
    underlying_decimals: usize,
    base_decimals: usize,
) -> TradeEventWithPrice {
    let price = prices.get_price(
        &pool_id,
        carmine_api_prices::BlockId::Timestamp(event.timestamp),
    );
    let capital_transfered_string = event.capital_transfered.to_string();
    let capital_transfered_human_readable =
        string_to_float(&capital_transfered_string, underlying_decimals);
    let tokens_minted_string = event.tokens_minted.to_string();
    let tokens_minted_human_readable = string_to_float(&tokens_minted_string, base_decimals);
    let capital_transfered_usd = (capital_transfered_human_readable as f32) * price;

    let premia = calculate_premia(
        event.option_side,
        event.option_type,
        capital_transfered_human_readable,
        tokens_minted_human_readable,
        event.strike_price,
    );
    let premia_usd = (premia as f32) * price;

    TradeEventWithPrice {
        timestamp: event.timestamp,
        action: event.action.to_string(),
        caller: event.caller.to_string(),
        capital_transfered: capital_transfered_human_readable,
        capital_transfered_usd,
        underlying_asset_price_usd: price,
        tokens_minted: tokens_minted_human_readable,
        premia,
        premia_usd,
        option_side: event.option_side,
        option_type: event.option_type,
        maturity: event.maturity,
        strike_price: event.strike_price,
        pool_id: pool_id.to_string(),
    }
}

pub fn get_trades(
    trades_map: &HashMap<String, Vec<TradeEvent>>,
    prices: &HistoricalPrices,
) -> Trades {
    let mut all_trades = vec![];
    for (pool_id, trades) in trades_map.into_iter() {
        let underlying_decimals = pool_id_to_decimals(pool_id);
        let base_decimals = pool_id_to_base_decimals(pool_id);

        let pool_trades_with_price: Vec<TradeEventWithPrice> = trades
            .iter()
            .map(|t| transform_trade_event(t, prices, pool_id, underlying_decimals, base_decimals))
            .collect();

        all_trades.extend(pool_trades_with_price);
    }

    let mut user_trades: HashMap<String, Vec<TradeEventWithPrice>> = HashMap::new();

    for trade in &all_trades {
        user_trades
            .entry(trade.caller.clone())
            .or_insert_with(Vec::new)
            .push(trade.clone());
    }

    Trades {
        all_trades,
        user_trades,
    }
}
