use crate::schema::events;
use crate::schema::options;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(table_name = events)]
pub struct Event {
    pub block_hash: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub event_index: i64,
    pub from_address: String,
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub token_address: String,
    pub capital_transfered: String,
    pub tokens_minted: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(table_name = options)]
pub struct IOption {
    pub option_side: i16,
    pub maturity: i64,
    pub strike_price: String,
    pub quote_token_address: String,
    pub base_token_address: String,
    pub option_type: i16,
    pub option_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeHistory {
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub capital_transfered: String,
    pub tokens_minted: String,
    pub option: Option<IOption>,
    pub liquidity_pool: Option<String>,
}

pub struct AppData {
    pub all_non_expired: Vec<String>,
    pub trade_history: Vec<TradeHistory>,
}

pub struct AppState {
    pub mainnet: AppData,
    pub testnet: AppData,
}
