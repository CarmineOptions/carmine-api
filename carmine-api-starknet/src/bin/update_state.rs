use std::env;

use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let state_updater = AmmStateObserver::new();

    let n1 = 40138;

    let res = state_updater.update_single_block(n1).await;
    match res {
        Ok(_) => println!("Updated state for block {}", n1),
        Err(_) => panic!("Failed updating block"),
    }

    // for n in 41432..=41434 {
    //     let res = state_updater.update_single_block(n).await;
    //     match res {
    //         Ok(_) => println!("Updated state for block {}", n),
    //         Err(_) => panic!("Failed updating block"),
    //     }
    // }
}
