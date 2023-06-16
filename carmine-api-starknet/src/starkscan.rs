use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use carmine_api_core::{
    network::{amm_address, starkscan_base_url, Network, Protocol},
    types::{Event, StarkScanEvent, StarkScanEventResult},
};
use carmine_api_db::create_batch_of_events;
use reqwest::{Client, Error, Response};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

// 1. 3. 2023
const CUTOFF_TIMESTAMP: i64 = 1677625200;
const STARKSCAN_REQUESTS_DELAY_IN_MS: u64 = 1000;

// list of action names that will be stored
const ALLOWED_ACTIONS: [&'static str; 5] = [
    "TradeOpen",
    "TradeClose",
    "TradeSettle",
    "DepositLiquidity",
    "WithdrawLiquidity",
];

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

pub fn api_url(network: &Network, protocol: &Protocol) -> String {
    let base = starkscan_base_url(&network);
    let from_address = match protocol {
        Protocol::CarmineOptions => amm_address(&network),
        Protocol::Hashstack => "0x03dcf5c72ba60eb7b2fe151032769d49dd3df6b04fa3141dffd6e2aa162b7a6e",
        Protocol::ZkLend => "0x04c0a5193d58f74fbace4b74dcf65481e734ed1714121bdc571da345540efa05",
    };
    format!("{}?from_address={}&limit=100", base, from_address)
}

pub async fn api_call(url: &str) -> Result<Response, Error> {
    let api_key = env::var("STARKSCAN_API_KEY").expect("Failed to read API key");
    let mut headers = reqwest::header::HeaderMap::new();
    let client = Client::new();

    headers.insert("accept", "applicationjson".parse().unwrap());
    headers.insert("x-api-key", api_key.parse().unwrap());

    client.get(url).headers(headers).send().await
}

pub async fn api_call_json<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    let res = api_call(url).await?;
    let parsed_result = res.json::<T>().await;
    parsed_result
}

pub async fn api_call_text(url: &str) -> Result<String, Error> {
    let res = api_call(url).await?;
    res.text().await
}

pub async fn carmine_events_call(url: &str) -> Result<StarkScanEventResult, Error> {
    api_call_json::<StarkScanEventResult>(url).await
}

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
    let mut current_url = api_url(network, &Protocol::CarmineOptions);
    let mut count = 0;

    'data: loop {
        let res = match carmine_events_call(&current_url).await {
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
    let mut current_url = api_url(network, &Protocol::CarmineOptions);

    'data: loop {
        let res = match carmine_events_call(&current_url).await {
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
