use std::env;

use carmine_api_core::network::Network;
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;
use starknet::providers::{jsonrpc::JsonRpcClient, Provider};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let carmine = Carmine::new(Network::Mainnet);
    let option_info = carmine
        .get_option_info_from_addresses(
            "0x2f1ff24d90c395622a9db14ee2ba65be07d5238843c3188af42686d2144c3f3",
        )
        .await;

    let res = Provider::call_contract();

    if let Ok(opt) = option_info {
        println!("{:#?}\n\n\n", opt);

        let vol = carmine
            .get_option_volatility_and_position(opt, 195000, true)
            .await;
        println!("\nVolatility:\n{:?}", vol);
    }
}
