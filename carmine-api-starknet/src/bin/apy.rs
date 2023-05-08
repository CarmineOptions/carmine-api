use std::{str::FromStr, time::Duration};

use carmine_api_core::network::{call_lp_address, put_lp_address, Network};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;
use starknet::core::types::FieldElement;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = Network::Mainnet;
    let c = Carmine::new(network);
    let lp_address = FieldElement::from_str(call_lp_address(&network)).unwrap();

    let point_of_interes = 48766;
    let mut n = point_of_interes - 30;

    while n < point_of_interes + 30 {
        if let Ok(v) = c.get_value_of_lp_token(n, lp_address).await {
            println!("block #{} --- call_lp_token_value {}", n, v);
            n = n + 5;
        } else {
            sleep(Duration::from_secs(3)).await;
        }
    }
}
