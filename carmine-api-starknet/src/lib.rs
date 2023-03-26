mod starkscan;

use carmine_api_core::{Event, IOption};
use futures::future::join_all;
use starknet::core::types::{CallContractResult, CallFunction, FieldElement};
use starknet::macros::selector;
use starknet::{
    self,
    core::types::BlockId,
    providers::{Provider, SequencerGatewayProvider},
};
use starkscan::StarkScanEvent;
use std::time::Duration;
use tokio::time::sleep;

use crate::starkscan::parse_event;

// 1. 3. 2023
const CUTOFF_TIMESTAMP: i64 = 1677625200;

fn format_call_contract_result(res: CallContractResult) -> Vec<String> {
    let mut arr: Vec<String> = vec![];

    // first element is length of the result - skip it
    for v in res.result.into_iter().skip(1) {
        let base_10 = format!("{}", v);
        arr.push(base_10);
    }

    arr
}

pub struct Carmine {
    provider: SequencerGatewayProvider,
}

impl Carmine {
    // TODO: everything is hardcoded for the testnet!
    const CALL_LP_ADDRESS: &str =
        "0x03b176f8e5b4c9227b660e49e97f2d9d1756f96e5878420ad4accd301dd0cc17";
    const PUT_LP_ADDRESS: &str =
        "0x0030fe5d12635ed696483a824eca301392b3f529e06133b42784750503a24972";
    const CONTRACT_ADDRESS: &str =
        "0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54";

    pub fn new() -> Self {
        let provider = SequencerGatewayProvider::starknet_alpha_goerli();

        Carmine { provider }
    }

    pub async fn get_all_non_expired_options_with_premia(&self) -> Vec<String> {
        let entrypoint = selector!("get_all_non_expired_options_with_premia");
        let call = self.provider.call_contract(
            CallFunction {
                contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                entry_point_selector: entrypoint,
                calldata: vec![FieldElement::from_hex_be(Carmine::CALL_LP_ADDRESS).unwrap()],
            },
            BlockId::Latest,
        );
        let put = self.provider.call_contract(
            CallFunction {
                contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                entry_point_selector: entrypoint,
                calldata: vec![FieldElement::from_hex_be(Carmine::PUT_LP_ADDRESS).unwrap()],
            },
            BlockId::Latest,
        );

        let contract_results = join_all(vec![call, put]).await;

        let mut fetched_data: Vec<String> = Vec::new();

        for result in contract_results {
            if let Ok(v) = result {
                let mut formatted = format_call_contract_result(v);
                fetched_data.append(&mut formatted);
            }
        }
        fetched_data
    }

    pub async fn get_option_info_from_addresses(
        &self,
        option_address: &str,
    ) -> Result<IOption, &str> {
        let entrypoint = selector!("get_option_info_from_addresses");
        let call = self.provider.call_contract(
            CallFunction {
                contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                entry_point_selector: entrypoint,
                calldata: vec![
                    FieldElement::from_hex_be(Carmine::CALL_LP_ADDRESS).unwrap(),
                    FieldElement::from_hex_be(option_address).unwrap(),
                ],
            },
            BlockId::Latest,
        );
        let put = self.provider.call_contract(
            CallFunction {
                contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                entry_point_selector: entrypoint,
                calldata: vec![
                    FieldElement::from_hex_be(Carmine::PUT_LP_ADDRESS).unwrap(),
                    FieldElement::from_hex_be(option_address).unwrap(),
                ],
            },
            BlockId::Latest,
        );

        let contract_results = join_all(vec![call, put]).await;

        for result in contract_results {
            if let Ok(call_res) = result {
                let data = call_res.result;
                assert_eq!(data.len(), 6, "Got wrong size Option result");

                let option_side = format!("{}", data[0])
                    .parse::<i16>()
                    .expect("Failed to parse side");
                let option_type = format!("{}", data[5])
                    .parse::<i16>()
                    .expect("Failed to parse type");
                let maturity = format!("{}", data[1])
                    .parse::<i64>()
                    .expect("Failed to parse maturity");
                let strike_price = format!("{:#x}", data[2]);
                let quote_token_address = format!("{:#x}", data[3]);
                let base_token_address = format!("{:#x}", data[4]);

                return Ok(IOption {
                    option_side,
                    option_type,
                    strike_price,
                    maturity,
                    quote_token_address,
                    base_token_address,
                    option_address: String::from(option_address),
                });
            }
        }

        Err("Failed to find option with given address")
    }

    pub async fn get_option_token_address(
        &self,
        lptoken_address: &str,
        option_side: FieldElement,
        maturity: FieldElement,
        strike_price: FieldElement,
    ) -> Result<String, &str> {
        let entrypoint = selector!("get_option_token_address");
        let contract_result = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                    entry_point_selector: entrypoint,
                    calldata: vec![
                        FieldElement::from_hex_be(lptoken_address).unwrap(),
                        option_side,
                        maturity,
                        strike_price,
                    ],
                },
                BlockId::Latest,
            )
            .await;

        match contract_result {
            Ok(v) => {
                let data = v.result[0];
                let address = format!("{:#x}", data);
                return Ok(address);
            }
            Err(e) => {
                println!("Failed \"get_option_token_address\" \n{}", e);
                return Err("Failed \"get_option_token_address\"");
            }
        }
    }

    async fn get_options_with_addresses_from_single_pool(
        &self,
        pool_address: &str,
    ) -> Result<Vec<IOption>, &str> {
        let entrypoint = selector!("get_all_options");
        let contract_result = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                    entry_point_selector: entrypoint,
                    calldata: vec![FieldElement::from_hex_be(pool_address).unwrap()],
                },
                BlockId::Latest,
            )
            .await;

        let data: Vec<FieldElement> = match contract_result {
            Err(provider_error) => {
                println!("{:?}", provider_error);
                return Err("Failed calling endpoint \"get_all_options\"");
            }
            Ok(v) => {
                let mut res = v.result;
                // first element is length of result array - remove it
                res.remove(0);

                res
            }
        };

        // each option has 6 fields
        let chunks = data.chunks(6);

        let mut options: Vec<IOption> = vec![];

        for option_vec in chunks {
            if option_vec.len() != 6 {
                println!("Wrong option_vec size!");
                continue;
            }

            // avoid running into rate limit starknet error
            sleep(Duration::from_secs(2)).await;

            let option_address_result = self
                .get_option_token_address(pool_address, option_vec[0], option_vec[1], option_vec[2])
                .await;

            let option_address = match option_address_result {
                Err(e) => {
                    println!("Failed to get option address\n{}", e);
                    continue;
                }
                Ok(v) => v.to_lowercase(),
            };

            let option_side = format!("{}", option_vec[0])
                .parse::<i16>()
                .expect("Failed to parse side");
            let option_type = format!("{}", option_vec[5])
                .parse::<i16>()
                .expect("Failed to parse type");
            let maturity = format!("{}", option_vec[1])
                .parse::<i64>()
                .expect("Failed to parse maturity");
            let strike_price = format!("{:#x}", option_vec[2]);
            let quote_token_address = format!("{:#x}", option_vec[3]);
            let base_token_address = format!("{:#x}", option_vec[4]);

            let option = IOption {
                option_side,
                maturity,
                strike_price,
                quote_token_address,
                base_token_address,
                option_type,
                option_address,
            };

            options.push(option);
        }

        Ok(options)
    }

    /// This method returns all options, addresses included.
    /// !This method is extremely slow, because it waits 2s between
    /// Starknet calls to avoid running into "rate limit" error!
    pub async fn get_options_with_addresses(&self) -> Vec<IOption> {
        let call = self.get_options_with_addresses_from_single_pool(Carmine::CALL_LP_ADDRESS);
        let put = self.get_options_with_addresses_from_single_pool(Carmine::PUT_LP_ADDRESS);
        let contract_results = join_all(vec![call, put]).await;

        let mut options: Vec<IOption> = vec![];

        for result in contract_results {
            if let Ok(mut v) = result {
                options.append(&mut v);
            }
        }

        println!("Got options from Starknet");

        options
    }
}

pub async fn get_events_from_starkscan() -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    let mut current_url = String::from("https://api-testnet.starkscan.co/api/v0/events?from_address=0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54");

    let mut count = 0;

    'data: loop {
        let res = match starkscan::api_call(&current_url).await {
            Ok(v) => v,
            Err(_) => {
                break 'data;
            }
        };

        count = count + 1;

        let data = res.data;

        for event in data {
            // only check events up to this timestamp
            // every next event is just as old or older
            // therefore it is safe to break top loop
            if event.timestamp < CUTOFF_TIMESTAMP {
                break 'data;
            }

            if let Some(parsed_event) = parse_event(event) {
                events.push(parsed_event);
            }
        }

        if let Some(next_url) = res.next_url {
            current_url = next_url;
        } else {
            break 'data;
        }
        // do not exceed API usage limit (3 rps)
        sleep(Duration::from_millis(340)).await;
    }

    println!("Got events from Starkscan with {} requests", count);

    events
}

// TODO: abstract to remove code duplicity
pub async fn get_new_events_from_starkscan(stored_events: &Vec<Event>) -> Vec<Event> {
    // collection of already stored TXs
    let stored_txs: Vec<String> = stored_events
        .into_iter()
        .map(|e| String::from(&e.transaction_hash))
        .collect();
    let mut new_events: Vec<Event> = Vec::new();

    let mut count = 0;
    let mut current_url = String::from("https://api-testnet.starkscan.co/api/v0/events?from_address=0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54");

    'data: loop {
        let res = match starkscan::api_call(&current_url).await {
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
            if event.timestamp < CUTOFF_TIMESTAMP {
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
        // do not exceed API usage limit (3 rps)
        sleep(Duration::from_millis(340)).await;
    }

    println!(
        "Fetched {} previously not stored events with {} requests",
        new_events.len(),
        count
    );

    new_events
}
