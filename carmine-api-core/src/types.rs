use core::fmt;
use std::{collections::HashMap, time::SystemTime};

use crate::schema::{
    blocks, braavos_bonus, events, insurance_events, options, options_volatility, oracle_prices,
    pool_state, pools, referral_codes, referral_events, starkscan_events,
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

#[derive(Debug, Clone, Serialize)]
pub struct TradeEvent {
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub capital_transfered: String,
    pub tokens_minted: String,
    pub option_side: i16,
    pub option_type: i16,
    pub maturity: i64,
    pub strike_price: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeEventWithPrice {
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub capital_transfered: f64,
    pub capital_transfered_usd: f32,
    pub underlying_asset_price_usd: f32,
    pub tokens_minted: f64,
    pub premia: f64,
    pub premia_usd: f32,
    pub option_side: i16,
    pub option_type: i16,
    pub maturity: i64,
    pub strike_price: f64,
    pub pool_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StakeEvent {
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub capital_transfered: String,
    pub tokens_minted: String,
}

pub enum TokenPair {
    EthUsdc,
    BtcUsdc,
    StrkUsdc,
}

impl TokenPair {
    pub fn id(&self) -> String {
        match self {
            TokenPair::EthUsdc => "eth-usdc".to_string(),
            TokenPair::BtcUsdc => "btc-usdc".to_string(),
            TokenPair::StrkUsdc => "strk-usdc".to_string(),
        }
    }
}

impl fmt::Display for TokenPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenPair::EthUsdc => write!(f, "eth-usdc"),
            TokenPair::BtcUsdc => write!(f, "btc-usdc"),
            TokenPair::StrkUsdc => write!(f, "strk-usdc"),
        }
    }
}

pub enum OracleName {
    Pragma,
}

pub struct Trades {
    pub all_trades: Vec<TradeEventWithPrice>,
    pub user_trades: HashMap<String, Vec<TradeEventWithPrice>>,
}

pub struct AppData {
    pub all_non_expired: Vec<String>,
    pub trade_history: Vec<TradeHistory>,
    pub legacy_trade_history: Vec<TradeHistory>,
    pub trades: HashMap<String, Vec<TradeEvent>>,
    pub option_volatility: Vec<OptionWithVolatility>,
    pub state: HashMap<String, Vec<PoolStateWithTimestamp>>,
    pub oracle_prices: HashMap<String, Vec<OraclePriceConcise>>,
    pub apy: HashMap<String, APY>,
    pub referrals: Vec<ReferralEventDigest>,
    pub top_user_points: Vec<UserPointsWithPosition>,
    pub user_points: HashMap<String, UserPointsWithPosition>,
    pub votes: Vec<Vote>,
    pub votes_map: HashMap<String, Vec<Vote>>,
    pub defispring: DefispringInfo,
    pub braavos_proscore: HashMap<String, BraavosBonusValues>,
    pub trades_with_prices: Trades,
    pub insurance_events: Vec<InsuranceData>,
}

pub struct AppState {
    pub mainnet: AppData,
    pub airdrop: MerkleTree,
    pub token_prices: TokenPrices,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarkScanEventResult {
    pub next_url: Option<String>,
    pub data: Option<Vec<StarkScanEvent>>,
    pub message: Option<String>,
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
#[diesel(table_name = starkscan_events)]
pub struct StarkScanEventSettled {
    pub id: String,
    pub block_hash: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub event_index: i64,
    pub from_address: String,
    pub keys: Vec<String>,
    pub data: Vec<String>,
    pub timestamp: i64,
    pub key_name: String,
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
    pub lp_token_value_usd: Option<f64>,
    pub underlying_asset_price: Option<f64>,
    pub block_number: i64,
    pub lp_address: String,
}

#[derive(Debug, Serialize)]
pub struct PoolStatePriceUpdate {
    pub lp_token_value_usd: f64,
    pub underlying_asset_price: f64,
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
    pub lp_token_value_usd: Option<f64>,
    pub underlying_asset_price: Option<f64>,
    pub block_number: i64,
    pub lp_address: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, PartialEq, Selectable)]
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

#[derive(
    AsChangeset, Associations, Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable,
)]
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

#[derive(Serialize, Debug, Clone, Copy)]
pub struct OraclePriceConcise {
    pub price: i64,
    pub decimals: i16,
    pub last_updated_timestamp: i64,
    pub block_number: i64,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = referral_codes)]
pub struct ReferralCode {
    pub wallet_address: String,
    pub referral_code: String,
}

#[derive(Debug, Clone, Queryable, Serialize, Selectable)]
#[diesel(table_name = referral_events)]
pub struct ReferralEvent {
    pub id: i32,
    pub referred_wallet_address: String,
    pub referral_code: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferralEventDigest {
    pub referred_wallet_address: String,
    pub referee_wallet_address: String,
    pub referral_code: String,
    pub timestamp: SystemTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = referral_events)]
pub struct NewReferralEvent<'a> {
    pub referred_wallet_address: &'a str,
    pub referral_code: &'a str,
}

#[derive(Debug, Serialize, Default)]
pub struct APY {
    pub week: f64,
    pub week_annualized: f64,
    pub launch: f64,
    pub launch_annualized: f64,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = insurance_events)]
pub struct InsuranceEvent<'a> {
    pub user_address: &'a str,
    pub calldata: Vec<&'a str>,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct InsuranceEventQueryable {
    pub id: i32,
    pub user_address: String,
    pub calldata: Vec<String>,
    pub timestamp: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceData {
    pub user_address: String,
    pub base_token_price: f32,
    pub timestamp: i64,
    pub base_token_address: String,
    pub premia: f64,
    pub strike: f64,
    pub size: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserPointsDb {
    pub id: i32,
    pub user_address: String,
    pub timestamp: SystemTime,
    pub trading_points: i64,
    pub liquidity_points: i64,
    pub referral_points: i64,
    pub vote_points: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserPoints {
    pub address: String,
    pub trading_points: i64,
    pub liquidity_points: i64,
    pub referral_points: i64,
    pub vote_points: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserPointsWithPosition {
    pub address: String,
    pub trading_points: i64,
    pub liquidity_points: i64,
    pub referral_points: i64,
    pub vote_points: i64,
    pub total_points: i64,
    pub position: i64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Vote {
    pub user_address: String,
    pub prop_id: usize,
    pub opinion: usize,
    pub timestamp: i64,
}

#[derive(Queryable, Debug)]
pub struct PoolTvlInfo {
    pub block_number: i64,
    pub timestamp: i64,
    pub lp_address: String,
    pub option_positions: Vec<String>,
    pub unlocked_capital: String,
    pub locked_capital: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinGeckoPrice {
    pub usd: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceResponse {
    pub ethereum: CoinGeckoPrice,
    #[serde(rename = "usd-coin")]
    pub usd_coin: CoinGeckoPrice,
    pub starknet: CoinGeckoPrice,
    pub bitcoin: CoinGeckoPrice,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TokenPrices {
    pub eth: f64,
    pub usdc: f64,
    pub strk: f64,
    pub btc: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DefispringInfo {
    pub tvl: f64,
    pub allocation: f64,
    pub apy: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenblockData {
    pub date: String,
    pub protocol: String,
    pub allocation: f64,
    pub tvl: f64,
    pub volumes: f64,
    pub beta_fees: f64,
    pub apr: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenblockResponse {
    #[serde(rename = "Carmine")]
    pub carmine: Vec<OpenblockData>,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = braavos_bonus)]
pub struct BraavosBonus {
    pub user_address: String,
    pub pro_score_80: Option<i64>,
    pub braavos_referral: Option<i64>,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct BraavosBonusValues {
    pub pro_score_80: Option<i64>,
    pub braavos_referral: Option<i64>,
}

#[derive(Serialize)]
pub struct PailToken {
    pub name: String,
    pub description: String,
    pub token_id: u64,
    pub image: String,
}
