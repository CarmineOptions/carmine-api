use std::collections::HashMap;

use carmine_api_core::{
    pool::pool_id_to_decimals,
    types::{TradeEvent, TradeEventWithPrice, Trades},
    utils::tokens_to_usd,
};
use carmine_api_prices::HistoricalPrices;

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
                let capital_transfered = t.capital_transfered.to_string();
                let capital_transfered_usd = tokens_to_usd(&capital_transfered, decimals, price);

                TradeEventWithPrice {
                    timestamp: t.timestamp,
                    action: t.action.to_string(),
                    caller: t.caller.to_string(),
                    capital_transfered,
                    capital_transfered_usd,
                    underlying_asset_price_usd: price,
                    tokens_minted: t.tokens_minted.to_string(),
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
