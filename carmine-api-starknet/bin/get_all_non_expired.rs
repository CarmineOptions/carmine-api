use std::env;

use carmine_api_starknet::Carmine;

#[tokio::main]
async fn main() {
    env::set_var("NETWORK", "mainnet");
    let c = Carmine::new();
    let res = c.get_all_non_expired_options_with_premia().await;
    println!("{:?}", res);
}
