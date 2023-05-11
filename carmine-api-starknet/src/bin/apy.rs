use std::str::FromStr;

use carmine_api_core::network::{call_lp_address, Network};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;
use starknet::core::types::FieldElement;
use tokio::try_join;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = Network::Mainnet;
    let c = Carmine::new(network);
    let lp_address = FieldElement::from_str(call_lp_address(&network)).unwrap();

    for n in 41432..41500 {
        if let Ok(_) = c.get_value_of_pool_position(n, lp_address).await {
            println!("{} worked", n);
        } else {
            println!("{} failed", n);
        }
    }

    // match try_join!(
    //     // c.get_pool_locked_capital(41432, lp_address),
    //     // c.get_unlocked_capital(41432, lp_address),
    //     // c.get_lpool_balance(41432, lp_address),
    //     c.get_value_of_pool_position(41432, lp_address),
    //     c.get_value_of_lp_token(41432, lp_address),
    // ) {
    //     Ok(_) => println!("It worked fine!"),
    //     Err(e) => println!("{:?}", e),
    // }
}
