use carmine_api_starknet::oracle::{Oracle, OracleName};
use starknet::core::types::BlockId;

#[tokio::main]
async fn main() {
    let pragma = Oracle::new(OracleName::Pragma);

    pragma.get_spot_median(BlockId::Number(73605)).await;
}
