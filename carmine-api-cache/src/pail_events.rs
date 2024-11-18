use std::collections::HashMap;

use carmine_api_core::types::{
    PailEvents, PailHedgeFinalized, PailHedgeOpen, StarkScanEventSettled,
};

fn transform_event(event: &StarkScanEventSettled) -> PailEvents {
    match event.key_name.as_str() {
        // #[derive(Drop, starknet::Event)]
        // struct HedgeOpenedEvent {
        //     #[key]
        //     user: ContractAddress,
        //     hedge_token_id: u256,            index 0, 1
        //     amount: u256,                    index 2, 3
        //     quote_token: ContractAddress,    index 4
        //     base_token: ContractAddress,     index 5
        //     maturity: u64,                   index 6
        //     at_price: Fixed                  index 7, 8
        // }
        "hedge_open" => {
            let e = PailHedgeOpen {
                user: event
                    .keys
                    .get(1)
                    .expect("Failed to get PAIL event user address")
                    .to_owned(),
                hedge_token_id: u64::from_str_radix(
                    &event.data.get(0).expect("Failed to get pail token id")[2..],
                    16,
                )
                .expect("Failed to parse pail token id"),
                amount: event
                    .data
                    .get(2)
                    .expect("Failed to get pail amount")
                    .to_string(),
                quote_token: event
                    .data
                    .get(4)
                    .expect("Failed to get pail quote_token")
                    .to_string(),
                base_token: event
                    .data
                    .get(5)
                    .expect("Failed to get pail base_token")
                    .to_owned(),
                maturity: u64::from_str_radix(
                    &event.data.get(7).expect("Failed to get pail maturity")[2..],
                    16,
                )
                .expect("Failed to parse pail maturity"),
                at_price: event
                    .data
                    .get(7)
                    .expect("Failed to get pail at price")
                    .to_string(),
                event: "hedge_open".to_string(),
            };
            PailEvents::Open(e)
        }
        // #[derive(Drop, starknet::Event)]
        // struct HedgeFinalizedEvent {
        //     #[key]
        //     user: ContractAddress,
        //     hedge_token_id: u256,    index 0, 1
        // }
        "hedge_close" => {
            let e = PailHedgeFinalized {
                user: event
                    .keys
                    .get(1)
                    .expect("Failed to get PAIL event user address")
                    .to_owned(),
                hedge_token_id: u64::from_str_radix(
                    &event.data.get(0).expect("Failed to get pail token id")[2..],
                    16,
                )
                .expect("Failed to parse pail token id"),
                event: "hedge_close".to_string(),
            };
            PailEvents::Close(e)
        }
        "hedge_settle" => {
            let e = PailHedgeFinalized {
                user: event
                    .keys
                    .get(1)
                    .expect("Failed to get PAIL event user address")
                    .to_owned(),
                hedge_token_id: u64::from_str_radix(
                    &event.data.get(0).expect("Failed to get pail token id")[2..],
                    16,
                )
                .expect("Failed to parse pail token id"),
                event: "hedge_settle".to_string(),
            };
            PailEvents::Settle(e)
        }
        _ => unreachable!("Unexpected PAIL event"),
    }
}

pub fn transform_pail_events(
    events: &Vec<StarkScanEventSettled>,
) -> HashMap<String, Vec<PailEvents>> {
    let mut res = HashMap::new();

    for event in events {
        let transformed_event = transform_event(event);
        res.entry(transformed_event.get_user())
            .or_insert_with(Vec::new)
            .push(transformed_event);
    }

    res
}
