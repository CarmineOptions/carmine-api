use std::env;

use carmine_api_core::types::Event;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

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

pub async fn api_call(url: &str) -> Result<StarkScanEventResult, Error> {
    let api_key = env::var("STARKSCAN_API_KEY").expect("Failed to read API key");
    let mut headers = reqwest::header::HeaderMap::new();
    let client = Client::new();

    headers.insert("accept", "applicationjson".parse().unwrap());
    headers.insert("x-api-key", api_key.parse().unwrap());

    let res = client.get(url).headers(headers).send().await?;

    let parsed_result = res.json::<StarkScanEventResult>().await;

    parsed_result
}

// list of action names that will be stored
const ALLOWED_ACTIONS: [&'static str; 5] = [
    "TradeOpen",
    "TradeClose",
    "TradeSettle",
    "DepositLiquidity",
    "WithdrawLiquidity",
];

pub fn parse_event(event: StarkScanEvent) -> Option<Event> {
    // if "key_name" is null or not allowed action (eg "ExpireOptionTokenForPool")
    // we can't handle the event so we don't store it
    let action = match event.key_name {
        Some(action) if ALLOWED_ACTIONS.iter().any(|&v| v == action) => action,
        _ => {
            return None;
        }
    };

    // block_hash and block_number can sometimes be null, skip if that's the case
    let block_hash = match event.block_hash {
        Some(hash) => hash,
        _ => {
            return None;
        }
    };
    // block_hash and block_number can sometimes be null, skip if that's the case
    let block_number = match event.block_number {
        Some(n) => n,
        _ => {
            return None;
        }
    };

    // accessing data by index, make sure the length is correct
    if event.data.len() != 6 {
        return None;
    }

    Some(Event {
        block_hash: block_hash,
        block_number: block_number,
        transaction_hash: event.transaction_hash,
        event_index: event.event_index,
        from_address: event.from_address,
        timestamp: event.timestamp,
        action: action,
        caller: String::from(&event.data[0]),
        token_address: String::from(&event.data[1]),
        capital_transfered: String::from(&event.data[2]),
        tokens_minted: String::from(&event.data[4]),
    })
}
