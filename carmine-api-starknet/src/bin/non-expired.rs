use carmine_api_core::network::Network;
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let carmine = Carmine::new(Network::Mainnet);
    match carmine.get_all_non_expired_options_with_premia().await {
        Ok(res) => println!("{:#?}", res),
        Err(e) => println!("FAILED! {:#?}", e),
    }
}
