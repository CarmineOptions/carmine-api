use carmine_api_core::network::Network;
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let carmine = Carmine::new(Network::Testnet);
    let res = carmine.get_all_non_expired_options_with_premia().await;

    println!("RESULT: {:#?}", res);
}
