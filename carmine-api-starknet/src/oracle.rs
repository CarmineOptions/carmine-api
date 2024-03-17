use std::collections::VecDeque;

use carmine_api_core::types::{DbBlock, OracleName, OraclePrice, TokenPair};
use carmine_api_rpc_gateway::{mainnet_call, BlockTag};

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
            (OracleName::Pragma, TokenPair::EthUsdc) => "0x4554482f555344".to_string(),
            (OracleName::Pragma, TokenPair::BtcUsdc) => "0x4254432f555344".to_string(),
            (OracleName::Pragma, TokenPair::StrkUsdc) => "0x5354524b2f555344".to_string(),
        }
    }

    pub async fn get_spot_median(
        &self,
        token_pair: &TokenPair,
        block: &DbBlock,
    ) -> Result<OraclePrice, String> {
        let entrypoint =
            "0x24b869ce68dd257b370701ca16e4aaf9c6483ff6805d04ba7661f3a0b6ce59".to_string(); // get_data_median
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
