use carmine_api_starknet::oracle::{Oracle, OracleName, TokenPair};

#[tokio::main]
async fn main() {
    let pragma = Oracle::new(OracleName::Pragma);

    let res = pragma.get_spot_median(TokenPair::EthUsdc, 74540).await;

    println!("{:?}", res);
}
