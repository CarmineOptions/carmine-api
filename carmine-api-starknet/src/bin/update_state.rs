use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state_updater = AmmStateObserver::new();

    let mut n = 475000;

    while n < 491903 {
        let res = state_updater.update_single_block(n).await;
        match res {
            Ok(_) => println!("Updated state for block {}", n),
            Err(_) => println!("Failed updating block"),
        }
        n += 1;
    }

    // for n in 41432..=41434 {
    //     let res = state_updater.update_single_block(n).await;
    //     match res {
    //         Ok(_) => println!("Updated state for block {}", n),
    //         Err(_) => panic!("Failed updating block"),
    //     }
    // }
}
