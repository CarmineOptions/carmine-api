use crate::types::TokenPair;

pub fn token_pair_id(token_pair: &TokenPair) -> String {
    match token_pair {
        TokenPair::EthUsdc => "eth-usdc".to_owned(),
    }
}
