use std::collections::HashMap;

use crate::schema::{
    blocks, events, options, options_volatility, oracle_prices, pool_state, pools,
};
use carmine_api_airdrop::merkle_tree::MerkleTree;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

pub enum TokenPair {
    EthUsdc,
}

pub enum OracleName {
    Pragma,
}

pub struct AppData {
    pub all_non_expired: Vec<String>,
    pub trade_history: Vec<TradeHistory>,
    pub option_volatility: Vec<OptionWithVolatility>,
    pub state_eth_usdc_call: Vec<PoolStateWithTimestamp>,
    pub state_eth_usdc_put: Vec<PoolStateWithTimestamp>,
    pub oracle_prices: HashMap<String, Vec<OraclePrice>>,
    pub apy_eth_usdc_call: f64,
    pub apy_eth_usdc_put: f64,
}

pub struct AppState {
    pub mainnet: AppData,
    pub testnet: AppData,
    pub airdrop: MerkleTree,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarkScanEventResult {
    pub next_url: Option<String>,
    pub data: Vec<StarkScanEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarkScanEvent {
    pub block_hash: Option<String>,
    pub block_number: Option<i64>,
    pub transaction_hash: String,
    pub event_index: i64,
    pub from_address: String,
    pub keys: Vec<String>,
    pub data: Vec<String>,
    pub timestamp: i64,
    pub key_name: Option<String>,
}

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

#[derive(Associations, Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(belongs_to(Pool, foreign_key = lp_address))]
#[diesel(table_name = options)]
pub struct IOption {
    pub option_side: i16,
    pub maturity: i64,
    pub strike_price: String,
    pub quote_token_address: String,
    pub base_token_address: String,
    pub option_type: i16,
    pub option_address: String,
    pub lp_address: String,
}

#[derive(Serialize)]
pub struct Volatility {
    pub block_number: i64,
    pub timestamp: i64,
    pub volatility: Option<String>,
    pub option_position: Option<String>,
}

#[derive(Serialize)]
pub struct OptionWithVolatility {
    pub option_side: i16,
    pub maturity: i64,
    pub strike_price: String,
    pub quote_token_address: String,
    pub base_token_address: String,
    pub option_type: i16,
    pub option_address: String,
    pub lp_address: String,
    pub volatilities: Vec<Volatility>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolStats {
    pub unlocked_cap: String,
    pub locked_cap: String,
    pub lp_balance: String,
    pub pool_position: String,
    pub lp_token_value: String,
}

#[derive(Associations, Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(belongs_to(Pool, foreign_key = lp_address))]
#[diesel(belongs_to(DbBlock, foreign_key = block_number))]
#[diesel(table_name = pool_state)]
pub struct PoolState {
    pub unlocked_cap: String,
    pub locked_cap: String,
    pub lp_balance: String,
    pub pool_position: Option<String>,
    pub lp_token_value: Option<String>,
    pub block_number: i64,
    pub lp_address: String,
}

#[derive(Debug, Serialize)]
pub struct PoolStateWithTimestamp {
    pub unlocked_cap: String,
    pub locked_cap: String,
    pub lp_balance: String,
    pub pool_position: Option<String>,
    pub lp_token_value: Option<String>,
    pub block_number: i64,
    pub lp_address: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(table_name = blocks)]
pub struct DbBlock {
    pub block_number: i64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(table_name = pools)]
pub struct Pool {
    pub lp_address: String,
}

#[derive(Associations, Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(belongs_to(IOption, foreign_key = option_address))]
#[diesel(belongs_to(DbBlock, foreign_key = block_number))]
#[diesel(table_name = options_volatility)]
pub struct OptionVolatility {
    pub option_address: String,
    pub block_number: i64,
    pub volatility: Option<String>,
    pub option_position: Option<String>,
}

#[derive(Associations, Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(belongs_to(DbBlock, foreign_key = block_number))]
#[diesel(table_name = oracle_prices)]
pub struct OraclePrice {
    pub id: String,
    pub token_pair: String,
    pub price: i64,
    pub decimals: i16,
    pub last_updated_timestamp: i64,
    pub num_sources_aggregated: i16,
    pub oracle_name: String,
    pub block_number: i64,
}
