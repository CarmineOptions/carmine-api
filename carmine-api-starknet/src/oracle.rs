use std::collections::VecDeque;

use carmine_api_core::types::{DbBlock, OracleName, OraclePrice, TokenPair};
use carmine_api_rpc_gateway::{mainnet_call, BlockTag};
use starknet::macros::selector;

pub struct Oracle {
    name: OracleName,
    oracle_name: &'static str,
    oracle_address: &'static str,
}

impl Oracle {
    pub fn new(oracle: OracleName) -> Self {
        let (oracle_address, oracle_name) = match &oracle {
            OracleName::Pragma => (
                "0x02a85bd616f912537c50a49a4076db02c00b29b2cdc8a197ce92ed1837fa875b",
                "pragma",
            ),
        };
        let name = oracle;

        Oracle {
            name,
            oracle_name,
            oracle_address,
        }
    }

    fn oracle_specific_token_pair_id(&self, token_pair: &TokenPair) -> String {
        match (&self.name, token_pair) {
            (OracleName::Pragma, TokenPair::EthUsdc) => "19514442401534788".to_string(),
            (OracleName::Pragma, TokenPair::BtcUsdc) => "18669995996566340".to_string(),
            (OracleName::Pragma, TokenPair::StrkUsdc) => "6004514686061859652".to_string(),
        }
    }

    pub async fn get_spot_median(
        &self,
        token_pair: &TokenPair,
        block: &DbBlock,
    ) -> Result<OraclePrice, String> {
        let entrypoint = selector!("get_data_median").to_string();
        let block_number = block.block_number;
        let block_tag = BlockTag::Number(block_number);
        let calldata = vec![
            "0".to_string(), // enum variation
            self.oracle_specific_token_pair_id(&token_pair),
        ];

        let res = mainnet_call(
            self.oracle_address.to_owned(),
            entrypoint,
            calldata,
            block_tag,
        )
        .await;

        // [
        //     "0x36324ad8ff", price
        //     "0x8", decimals
        //     "0x6593058a", last updated timestamp
        //     "0x8", sources
        //     "0x0",
        //     "0x0",
        // ]

        let err_msg = format!("Unexpected oracle call result {:?}", res);
        if let Ok(data) = res {
            if data.len() < 4 {
                return Err(format!(
                    "Incorrect number of fields in Oracle response: {:?}",
                    data
                ));
            }

            let stripped_data: VecDeque<&str> =
                data.iter().map(|s| s.trim_start_matches("0x")).collect();
            let mut deque = VecDeque::from(stripped_data);

            let price = i64::from_str_radix(deque.pop_front().unwrap(), 16)
                .expect("Failed parsing oracle price");
            let decimals = i16::from_str_radix(deque.pop_front().unwrap(), 16)
                .expect("Failed parsing oracle decimals");
            let last_updated_timestamp = i64::from_str_radix(deque.pop_front().unwrap(), 16)
                .expect("Failed parsing oracle timestamp");
            let num_sources_aggregated = i16::from_str_radix(deque.pop_front().unwrap(), 16)
                .expect("Failed parsing oracle sources");
            let id = format!("{}_{}_{}", block_number, token_pair, self.oracle_name);

            return Ok(OraclePrice {
                token_pair: token_pair.id(),
                id,
                price,
                decimals,
                last_updated_timestamp,
                num_sources_aggregated,
                block_number,
                oracle_name: self.oracle_name.to_string(),
            });
        }
        return Err(err_msg);
    }
}
