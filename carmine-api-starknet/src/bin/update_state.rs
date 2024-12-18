use carmine_api_starknet::amm_state::AmmStateObserver;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let so = AmmStateObserver::new();

    let mut n = 879200;
    let max = 879253;

    while n < max {
        match so.update_single_block(n).await {
            Ok(_) => println!("updated {}", n),
            Err(_) => println!("FAILED {}", n),
        };
        n += 5;
    }
}
