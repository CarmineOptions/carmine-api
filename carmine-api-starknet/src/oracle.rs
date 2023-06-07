use carmine_api_core::types::OraclePrice;
use starknet::{
    core::types::{BlockId, CallFunction, FieldElement},
    macros::selector,
    providers::{Provider, SequencerGatewayProvider},
};

pub enum TokenPair {
    EthUsdc,
}

pub enum OracleName {
    Pragma,
}

pub struct Oracle {
    provider: SequencerGatewayProvider,
    name: OracleName,
    oracle_name: String,
    oracle_address: FieldElement,
}

impl Oracle {
    pub fn new(oracle: OracleName) -> Self {
        let provider = SequencerGatewayProvider::starknet_alpha_mainnet();
        let (oracle_address, oracle_name) = match &oracle {
            OracleName::Pragma => (
                FieldElement::from_hex_be(
                    "0x0346c57f094d641ad94e43468628d8e9c574dcb2803ec372576ccc60a40be2c4",
                )
                .unwrap(),
                "pragma".to_owned(),
            ),
        };
        let name = oracle;

        Oracle {
            provider,
            name,
            oracle_name,
            oracle_address,
        }
    }

    fn pair_id(&self, token_pair: TokenPair) -> FieldElement {
        match (&self.name, token_pair) {
            (OracleName::Pragma, TokenPair::EthUsdc) => {
                FieldElement::from(19514442401534788 as u64)
            }
        }
    }

    pub async fn get_spot_median(
        &self,
        token_pair: TokenPair,
        block_number: i64,
    ) -> Result<OraclePrice, String> {
        let entrypoint = selector!("get_spot_median");
        let res = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: self.oracle_address,
                    entry_point_selector: entrypoint,
                    calldata: vec![self.pair_id(token_pair)],
                },
                BlockId::Number(block_number as u64),
            )
            .await;

        // Response format:
        // price 186825000000
        // decimals 8
        // last_updated_timestamp 1685974805
        // num_sources_aggregated 6

        let err_msg = format!("Unexpected oracle call result {:?}", res);
        if let Ok(call_result) = res {
            let data: Vec<FieldElement> = call_result.result;

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
                id,
                price,
                decimals,
                last_updated_timestamp,
                num_sources_aggregated,
                block_number,
                oracle_name: self.oracle_name.clone(),
            });
        }
        return Err(err_msg);
    }
}
