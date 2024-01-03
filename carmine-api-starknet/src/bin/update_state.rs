use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state_updater = AmmStateObserver::new();

    let mut n = 475225;

    while n < 491903 {
        let res = state_updater.update_single_block(n).await;
        match res {
            Ok(_) => println!("UPDATED {}", n),
            Err(_) => println!("FAILED {}", n),
        }
        n += 1;
    }
}
