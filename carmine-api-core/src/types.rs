use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Event {
    pub block_hash: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub event_index: i64,
    pub from_address: String,
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub option_address: String,
    pub capital_transfered: String,
    pub option_tokens_minted: String,
}

#[derive(Debug, Clone, Serialize)]
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
    pub option_tokens_minted: String,
    pub option: Option<IOption>,
    pub liquidity_pool: Option<String>,
}
