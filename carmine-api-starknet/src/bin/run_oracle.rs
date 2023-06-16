use carmine_api_core::types::{OracleName, TokenPair, DbBlock};
use carmine_api_starknet::oracle::Oracle;

#[tokio::main]
async fn main() {
    let pragma = Oracle::new(OracleName::Pragma);

    let res = pragma
        .get_spot_median(
            TokenPair::EthUsdc,
            &DbBlock {
                block_number: 75312,
                timestamp: 1686136008,
            },
        )
        .await;

    println!("{:?}", res);
}
