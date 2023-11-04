use std::{cmp::min, env, time::Duration};

use async_recursion::async_recursion;
use carmine_api_core::{
    network::{protocol_address, starkscan_base_url, Network, Protocol},
    telegram_bot,
    types::{Event, StarkScanEvent, StarkScanEventResult, StarkScanEventSettled},
};
use carmine_api_db::{
    create_batch_of_events, get_last_block_for_protocol_event, get_last_timestamp_carmine_event,
};
use reqwest::{Client, Error, Response};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

pub struct StarkscanUrlBuilder<'a> {
    url: String,
    first_param: bool,
    limit_set: bool,
    network: &'a Network,
}

impl<'a> StarkscanUrlBuilder<'a> {
    pub fn new(network: &'a Network) -> Self {
        StarkscanUrlBuilder {
            url: starkscan_base_url(&network).to_owned(),
            first_param: true,
            limit_set: false,
            network,
        }
    }
    fn append_param(&mut self, key: &str, value: &str) {
        let delimiter = match self.first_param {
            true => {
                self.first_param = false;
                "?"
            }
            false => "&",
        };
        self.url = format!("{}{}{}={}", self.url, delimiter, key, value);
    }
    fn append_num_param(&mut self, key: &str, n: u32) {
        self.append_param(key, n.to_string().as_str());
    }
    fn set_limit(&mut self, n: u8) {
        if !self.limit_set {
            self.append_num_param("limit", min(n as u32, 100));
            self.limit_set = true;
        }
    }
    pub fn protocol(mut self, protocol: &Protocol) -> Self {
        let from_address = protocol_address(&self.network, protocol);
        self.append_param("from_address", from_address);
        self
    }
    pub fn from_block(mut self, n: u32) -> Self {
        self.append_num_param("from_block", n);
        self
    }
    pub fn to_block(mut self, n: u32) -> Self {
        self.append_num_param("to_block", n);
        self
    }
    pub fn limit(mut self, n: u8) -> Self {
        self.set_limit(n);
        self
    }
    pub fn get_url(mut self) -> String {
        if !self.limit_set {
            self.set_limit(100);
        }
        self.url
    }
}

const STARKSCAN_REQUESTS_DELAY_IN_MS: u64 = 1000;

// list of action names that will be stored
const ALLOWED_ACTIONS: [&'static str; 5] = [
    "TradeOpen",
    "TradeClose",
    "TradeSettle",
    "DepositLiquidity",
    "WithdrawLiquidity",
];

pub async fn api_call(url: &str) -> Result<Response, Error> {
    let api_key = env::var("STARKSCAN_API_KEY").expect("Failed to read API key");
    let mut headers = reqwest::header::HeaderMap::new();
    let client = Client::new();

    headers.insert("accept", "application/json".parse().unwrap());
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

pub async fn events_call(url: &str) -> Result<StarkScanEventResult, Error> {
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

// TODO: move Carmine events to Starkscan_events and then this will replace parse_event
pub fn parse_settled_event(event: StarkScanEventSettled) -> Option<Event> {
    // if "key_name" is null or not allowed action (eg "ExpireOptionTokenForPool")
    // we can't handle the event so we don't store it
    if !ALLOWED_ACTIONS.iter().any(|&v| v == event.key_name) {
        return None;
    };

    // accessing data by index, make sure the length is correct
    if event.data.len() != 6 {
        return None;
    }

    Some(Event {
        block_hash: event.block_hash,
        block_number: event.block_number,
        transaction_hash: event.transaction_hash,
        event_index: event.event_index,
        from_address: event.from_address,
        timestamp: event.timestamp,
        action: event.key_name,
        caller: String::from(&event.data[0]),
        token_address: String::from(&event.data[1]),
        capital_transfered: String::from(&event.data[2]),
        tokens_minted: String::from(&event.data[4]),
    })
}

fn get_settled_event(event: StarkScanEvent) -> Option<StarkScanEventSettled> {
    if event.block_hash.is_some() && event.block_number.is_some() && event.key_name.is_some() {
        return Some(StarkScanEventSettled {
            id: format!("{}_{}", event.transaction_hash, event.event_index),
            block_hash: event.block_hash.unwrap(),
            block_number: event.block_number.unwrap(),
            transaction_hash: event.transaction_hash,
            event_index: event.event_index,
            from_address: event.from_address,
            keys: event.keys,
            data: event.data,
            timestamp: event.timestamp,
            key_name: event.key_name.unwrap(),
        });
    }
    return None;
}

#[async_recursion]
async fn _fetch_events(url: &str, data: &mut Vec<StarkScanEventSettled>, cutoff_timestamp: i64) {
    let starkscan_response = match events_call(url).await {
        Ok(v) => v,
        Err(e) => {
            // request failed, we cannot store partly fetched events, because that
            // would create hole in the data -> throw away incomplete events
            data.clear();
            println!("Error from StarkScan: {:?}, URL: {}", e, url);
            telegram_bot::send_message("Starkscan events fetching failed").await;
            return;
        }
    };
    let next_url_option = &starkscan_response.next_url;

    if let Some(message) = starkscan_response.message {
        // if message something went wrong
        // print and run same URL again
        println!("Starkscan returned message: {}", message);
        // prevent "limit exceeded"
        sleep(Duration::from_millis(STARKSCAN_REQUESTS_DELAY_IN_MS)).await;
        return _fetch_events(url, data, cutoff_timestamp).await;
    }

    if let Some(response_data) = starkscan_response.data {
        for event in response_data {
            if event.timestamp > cutoff_timestamp {
                if let Some(settled) = get_settled_event(event) {
                    data.push(settled);
                }
            } else {
                return;
            }
        }
    }

    if let Some(next_url) = next_url_option {
        // prevent "limit exceeded"
        sleep(Duration::from_millis(STARKSCAN_REQUESTS_DELAY_IN_MS)).await;
        return _fetch_events(next_url, data, cutoff_timestamp).await;
    }
}

pub async fn fetch_events(
    initial_url: String,
    cutoff_timestamp: i64,
) -> Vec<StarkScanEventSettled> {
    let mut data: Vec<StarkScanEventSettled> = vec![];
    _fetch_events(&initial_url, &mut data, cutoff_timestamp).await;
    println!("Fetching event, URL: {}, data: {}", initial_url, data.len());
    data
}

pub async fn get_events_from_starkscan() {
    // no longer updating events for testnet
    let network = &Network::Mainnet;
    let last_timestamp = match get_last_timestamp_carmine_event(network) {
        Some(t) => t,
        None => return,
    };

    let url = StarkscanUrlBuilder::new(network)
        .protocol(&Protocol::CarmineOptions)
        .get_url();

    let events = fetch_events(url, last_timestamp).await;
    let parsed_events: Vec<Event> = events
        .into_iter()
        .filter_map(|e| parse_settled_event(e))
        .collect();
    // update DB
    create_batch_of_events(&parsed_events, network);
    println!("Stored {} events from Starkscan", &parsed_events.len());
}

pub async fn get_protocol_events(
    network: &Network,
    protocol: &Protocol,
) -> Vec<StarkScanEventSettled> {
    let last_block_number: u32 = match get_last_block_for_protocol_event(network, protocol) {
        Some(t) => t.try_into().expect("Failed parsing block_number -> u32"),
        None => 0,
    };
    println!("Protocol: {}, last block: {}", protocol, last_block_number);
    let url = StarkscanUrlBuilder::new(&network)
        .protocol(protocol)
        .from_block(last_block_number)
        .get_url();
    fetch_events(url, 0).await
}

pub async fn get_block_range_events(
    protocol: &Protocol,
    network: &Network,
    from: u32,
    to: u32,
) -> Vec<StarkScanEventSettled> {
    // we want to fetch till there is no "next_url"
    let last_timestamp = 0;
    let url = StarkscanUrlBuilder::new(network)
        .protocol(protocol)
        .from_block(from)
        .to_block(to)
        .get_url();

    fetch_events(url, last_timestamp).await
}
