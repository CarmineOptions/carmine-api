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
    id: String,
    oracle_address: FieldElement,
}

impl Oracle {
    pub fn new(oracle: OracleName) -> Self {
        let provider = SequencerGatewayProvider::starknet_alpha_mainnet();
        let (oracle_address, id) = match &oracle {
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
            id,
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

    pub async fn get_spot_median(&self, block_id: BlockId) {
        let entrypoint = selector!("get_spot_median");
        let res = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: self.oracle_address,
                    entry_point_selector: entrypoint,
                    calldata: vec![self.pair_id(TokenPair::EthUsdc)],
                },
                block_id,
            )
            .await;

        // Response format:
        // price 186825000000
        // decimals 8
        // last_updated_timestamp 1685974805
        // num_sources_aggregated 6

        if let Ok(call_result) = res {
            let data: Vec<FieldElement> = call_result.result;
            assert!(
                data.len() == 4,
                "Incorrect number of fields in Oracle response"
            );
            let price = data[0];
            let decimals = data[1];
            let last_updated_timestamp = data[2];
            let num_sources_aggregated = data[3];
            println!(
                "price: {}\ndecimals: {}\ntimestamp: {}\nsources: {}\nid: {}",
                price, decimals, last_updated_timestamp, num_sources_aggregated, self.id
            );
        }
    }
}
