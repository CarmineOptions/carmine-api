use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use carmine_api_core::{
    network::{amm_address, starkscan_base_url, Network},
    types::Event,
};
use carmine_api_db::create_batch_of_events;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

// 1. 3. 2023
const CUTOFF_TIMESTAMP: i64 = 1677625200;
const STARKSCAN_REQUESTS_DELAY_IN_MS: u64 = 1000;

fn cutoff_timestamp() -> i64 {
    match env::var("ENVIRONMENT") {
        Ok(v) if v == "local" => {
            // for local development
            // only fetch events from
            // last 24h
            let one_day_ago = SystemTime::now() - Duration::from_secs(24 * 60 * 60);
            one_day_ago.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
        _ => CUTOFF_TIMESTAMP,
    }
}

fn api_url(network: &Network) -> String {
    let base = starkscan_base_url(&network);
    let amm = amm_address(&network);
    format!("{}?from_address={}&limit=100", base, amm)
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

pub async fn get_events_from_starkscan(network: &Network) {
    let mut events: Vec<Event> = Vec::new();
    let mut current_url = api_url(network);
    let mut count = 0;

    'data: loop {
        let res = match api_call(&current_url).await {
            Ok(v) => v,
            Err(_) => {
                println!("Error from StarkScan");
                break 'data;
            }
        };
        count = count + 1;

        let data = res.data;

        for event in data {
            if let Some(parsed_event) = parse_event(event) {
                events.push(parsed_event);
            }
        }

        if let Some(next_url) = res.next_url {
            current_url = next_url;
        } else {
            break 'data;
        }
        sleep(Duration::from_millis(STARKSCAN_REQUESTS_DELAY_IN_MS)).await;
    }

    println!("Got events from Starkscan with {} requests", count);

    // update DB
    create_batch_of_events(&events, network);
}

// TODO: abstract to remove code duplicity
pub async fn get_new_events_from_starkscan(stored_events: &Vec<Event>, network: &Network) {
    // collection of already stored TXs
    let stored_txs: Vec<String> = stored_events
        .into_iter()
        .map(|e| String::from(&e.transaction_hash))
        .collect();
    let mut new_events: Vec<Event> = Vec::new();

    let mut count = 0;
    let mut current_url = api_url(network);

    'data: loop {
        let res = match api_call(&current_url).await {
            Ok(v) => v,
            Err(_) => {
                break 'data;
            }
        };
        count = count + 1;

        let data = res.data;

        let fetched_len = data.len();

        let filtered_events: Vec<StarkScanEvent> = data
            .into_iter()
            .filter(|strakscan_event| !stored_txs.contains(&strakscan_event.transaction_hash))
            .collect();

        let filtered_len = filtered_events.len();

        for event in filtered_events {
            // only check events up to this timestamp
            // every next event is just as old or older
            // therefore it is safe to break top loop
            if event.timestamp < cutoff_timestamp() {
                println!("Cutoff timestamp reached");
                break 'data;
            }

            if let Some(parsed_event) = parse_event(event) {
                new_events.push(parsed_event);
            }
        }

        if fetched_len != filtered_len {
            // reached TXs already stored in the DB - stop fetching
            break 'data;
        }

        if let Some(next_url) = res.next_url {
            current_url = next_url;
        } else {
            break 'data;
        }
        sleep(Duration::from_millis(STARKSCAN_REQUESTS_DELAY_IN_MS)).await;
    }

    println!(
        "Fetched {} previously not stored events with {} requests",
        new_events.len(),
        count
    );

    // update DB
    create_batch_of_events(&new_events, network);
}
