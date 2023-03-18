mod starkscan;

use carmine_api_db::models::NewEvent;
use futures::future::join_all;
use starknet::core::types::{CallContractResult, CallFunction, FieldElement};
use starknet::macros::{felt, selector};
use starknet::{
    self,
    core::types::BlockId,
    providers::{Provider, SequencerGatewayProvider},
};
use std::time::{Duration, SystemTime};
use tokio::time::sleep;

use crate::starkscan::parse_event;

fn format_call_contract_result(res: CallContractResult) -> Vec<String> {
    let mut arr: Vec<String> = vec![];

    // first element is length of the result - skip it
    for v in res.result.into_iter().skip(1) {
        let base_10 = format!("{}", v);
        arr.push(base_10);
    }

    arr
}

fn lp_address_to_call_function(lp_address: &str) -> CallFunction {
    CallFunction {
        contract_address: felt!(
            "0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54"
        ),
        entry_point_selector: selector!("get_all_non_expired_options_with_premia"),
        calldata: vec![FieldElement::from_hex_be(lp_address).unwrap()],
    }
}

#[derive(Debug)]
pub struct IOption {
    pub option_side: u8,
    pub maturity: i64,
    pub strike_price: String,
    pub quote_token_address: String,
    pub base_token_address: String,
    pub option_type: u8,
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
        let call = self.provider.call_contract(
            lp_address_to_call_function(
                "0x03b176f8e5b4c9227b660e49e97f2d9d1756f96e5878420ad4accd301dd0cc17",
            ),
            BlockId::Latest,
        );
        let put = self.provider.call_contract(
            lp_address_to_call_function(
                "0x0030fe5d12635ed696483a824eca301392b3f529e06133b42784750503a24972",
            ),
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

        println!("Fetched Vec length {}", fetched_data.len());

        fetched_data
    }

    pub async fn get_option_info_from_addresses(
        &self,
        option_token_address: &str,
    ) -> Result<IOption, &str> {
        let entrypoint = selector!("get_option_info_from_addresses");
        let call = self.provider.call_contract(
            CallFunction {
                contract_address: FieldElement::from_hex_be(Carmine::CONTRACT_ADDRESS).unwrap(),
                entry_point_selector: entrypoint,
                calldata: vec![
                    FieldElement::from_hex_be(Carmine::CALL_LP_ADDRESS).unwrap(),
                    FieldElement::from_hex_be(option_token_address).unwrap(),
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
                    FieldElement::from_hex_be(option_token_address).unwrap(),
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
                    .parse::<u8>()
                    .expect("Failed to parse side");
                let option_type = format!("{}", data[5])
                    .parse::<u8>()
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
                });
            }
        }

        Err("Failed to find option with given address")
    }

    pub async fn get_latest_block_id(&self) -> Option<u64> {
        let get_block_result = self.provider.get_block(BlockId::Latest).await;
        let block = match get_block_result {
            Ok(block) => block,
            Err(error) => panic!("Failed getting block {:?}", error),
        };

        block.block_number
    }

    pub async fn get_block(&self) {
        let get_block_result = self.provider.get_block(BlockId::Latest).await;

        let block = match get_block_result {
            Ok(block) => block,
            Err(error) => panic!("Failed getting block {:?}", error),
        };

        println!(
            "Received block with hash {}, timestamp {} blocknumber {}",
            block.block_hash.unwrap(),
            block.timestamp,
            block.block_number.unwrap()
        );
    }

    pub async fn get_blocks(&self) {
        let latest_block_id = self.get_latest_block_id().await.unwrap();

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let target_timestamp = now - 60 * 60 * 4; // 4 hours from now

        let mut i = latest_block_id;

        loop {
            println!("Exploring block #{}", i);
            let block_response = self.provider.get_block(BlockId::Number(i)).await;

            let block = match block_response {
                Ok(block) => block,
                Err(err) => {
                    println!("{:?}", err);
                    panic!("Failed to get block");
                }
            };

            i = i - 1;

            if block.timestamp < target_timestamp {
                println!(
                    "Block #{} is older than the target, there is total of {} blocks in this window",
                    i,
                    latest_block_id - i
                );
                break;
            }
        }

        println!("Done!");
    }
}

pub async fn get_events_from_starkscan() -> Vec<NewEvent> {
    let mut events: Vec<NewEvent> = Vec::new();

    // Date and time (GMT): Sunday 1. January 2023 0:00:00
    // 1672531200 timestamp in seconds
    let cutoff_timestamp = 1672531200; // TODO: change to the one above

    let mut current_url = String::from("https://api-testnet.starkscan.co/api/v0/events?from_address=0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54");
    let mut counter = 1;

    println!("Fetching events...");

    'data: loop {
        let res = starkscan::api_call(&current_url).await;
        for event in res.data {
            // only check events up to this timestamp
            // every next event is just as old or older
            // therefore it is safe to break top loop
            if event.timestamp < cutoff_timestamp {
                println!("Cutoff timestamp reached");
                break 'data;
            }

            if let Some(parsed_event) = parse_event(event) {
                events.push(parsed_event);
            }
        }

        if let Some(next_url) = res.next_url {
            println!("curl #{}, fetching next_url", counter);
            println!("Current events list {}", events.len());
            current_url = next_url;
        } else {
            println!("curl #{}, no next_url - we are done", counter);
            break 'data;
        }

        counter = counter + 1;

        // do not exceed API usage limit (3 rps)
        sleep(Duration::from_millis(340)).await;
    }

    events
}
