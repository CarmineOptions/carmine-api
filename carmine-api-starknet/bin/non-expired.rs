use carmine_api_core::network::Network;
use carmine_api_starknet::Carmine;

async fn non_expired_by_network(n: Network) {
    let c = Carmine::new(n);
    let res = c.get_all_non_expired_options_with_premia().await;
    println!("{}\n{:?}", n, res);
}

#[tokio::main]
async fn main() {
    non_expired_by_network(Network::Testnet).await;
    non_expired_by_network(Network::Mainnet).await;
}
