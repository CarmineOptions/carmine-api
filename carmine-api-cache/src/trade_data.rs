use std::collections::HashMap;

use carmine_api_core::{
    pool::pool_id_to_decimals,
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

pub fn get_trades(
    trades_map: &HashMap<String, Vec<TradeEvent>>,
    prices: &HistoricalPrices,
) -> Trades {
    let mut all_trades = vec![];
    for (pool_id, trades) in trades_map.into_iter() {
        let decimals = pool_id_to_decimals(pool_id);

        let pool_trades_with_price: Vec<TradeEventWithPrice> = trades
            .iter()
            .map(|t| {
                let price = prices.get_price(
                    &pool_id,
                    carmine_api_prices::BlockId::Timestamp(t.timestamp),
                );
                let capital_transfered_string = t.capital_transfered.to_string();
                let capital_transfered_human_readable =
                    string_to_float(&capital_transfered_string, decimals);
                let tokens_minted_string = t.tokens_minted.to_string();
                let tokens_minted_human_readable = string_to_float(&tokens_minted_string, 18); // tokens_minted has always 18 decimals
                let capital_transfered_usd = (capital_transfered_human_readable as f32) * price;

                let premia = calculate_premia(
                    t.option_side,
                    t.option_type,
                    capital_transfered_human_readable,
                    tokens_minted_human_readable,
                    t.strike_price,
                );
                let premia_usd = (premia as f32) * price;

                TradeEventWithPrice {
                    timestamp: t.timestamp,
                    action: t.action.to_string(),
                    caller: t.caller.to_string(),
                    capital_transfered: capital_transfered_human_readable,
                    capital_transfered_usd,
                    underlying_asset_price_usd: price,
                    tokens_minted: tokens_minted_human_readable,
                    premia,
                    premia_usd,
                    option_side: t.option_side,
                    option_type: t.option_type,
                    maturity: t.maturity,
                    strike_price: t.strike_price,
                    pool_id: pool_id.to_string(),
                }
            })
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
