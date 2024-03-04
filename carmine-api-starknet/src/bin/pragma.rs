use carmine_api_core::types::{DbBlock, OracleName, TokenPair};
use carmine_api_starknet::oracle::Oracle;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pragma = Oracle::new(OracleName::Pragma);

    let mut n = 599000;
    while n < 599900 {
        let block = DbBlock {
            block_number: n,
            timestamp: 1704134046,
        };

        match pragma.get_spot_median(TokenPair::StrkUsdc, &block).await {
            Ok(data) => println!("{} OK, data: {:#?}", n, data),
            Err(e) => println!("{} Failed, error: {:#?}", n, e),
        }
        n += 100;
    }
}
