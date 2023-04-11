use carmine_api_core::network::Network;
use carmine_api_starknet::Carmine;

#[tokio::main]
async fn main() {
    let c = Carmine::new(Network::Mainnet);
    let res = c.get_all_non_expired_options_with_premia().await;
    println!("{:?}", res);
}
