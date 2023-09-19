use std::env;

use carmine_api_starknet::{amm_state::AmmStateObserver, carmine::Carmine};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let call_pool = "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024";
    let put_pool = "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a";

    let carmine = Carmine::new(carmine_api_core::network::Network::Mainnet);

    for i in 32945..33000 {
        let v = carmine.get_value_of_lp_token(i, put_pool.to_string()).await;

        println!("{}: {:#?}", i, v);
    }

    // let bad_blocks: Vec<i64> = vec![];

    // let observer = AmmStateObserver::new();

    // for block_number in bad_blocks {
    //     match observer.update_single_block(block_number).await {
    //         Ok(_) => {
    //             println!("{} OK", block_number);
    //         }
    //         Err(_) => {
    //             println!("FAILED {}", block_number);
    //         }
    //     }
    // }

    println!("DONE");
}
