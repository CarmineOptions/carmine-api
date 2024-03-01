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
            (OracleName::Pragma, TokenPair::EthUsdc) => "19514442401534788".to_string(),
        }
    }

    pub async fn get_spot_median(
        &self,
        token_pair: TokenPair,
        block: &DbBlock,
    ) -> Result<OraclePrice, String> {
        let entrypoint = "get_spot_median".to_string();
        let block_number = block.block_number;
        let block_tag = BlockTag::Number(block_number);
        let calldata = vec![self.oracle_specific_token_pair_id(&token_pair)];

        let res = mainnet_call(
            self.oracle_address.to_owned(),
            entrypoint,
            calldata,
            block_tag,
        )
        .await;

        // Response format:
        // price 186825000000
        // decimals 8
        // last_updated_timestamp 1685974805
        // num_sources_aggregated 6

        let err_msg = format!("Unexpected oracle call result {:?}", res);
        if let Ok(data) = res {
            if data.len() != 4 {
                return Err(format!(
                    "Incorrect number of fields in Oracle response: {:?}",
                    data
                ));
            }

            let price = format!("{}", data[0]).parse::<i64>().unwrap();
            let decimals = format!("{}", data[1]).parse::<i16>().unwrap();
            let last_updated_timestamp = format!("{}", data[2]).parse::<i64>().unwrap();
            let num_sources_aggregated = format!("{}", data[3]).parse::<i16>().unwrap();
            let id = format!("{}-{}", block_number, self.oracle_name);

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
