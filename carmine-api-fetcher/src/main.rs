use tokio::time::{sleep, Duration};

use carmine_api_core::telegram_bot;
use carmine_api_starknet::{update_database_amm_state, update_database_events};

const UPDATE_DELAY: u64 = 20;

#[actix_web::main]
async fn main() {
    loop {
        if let Err(err) = actix_web::rt::spawn(async { update_database_events().await }).await {
            // failed, probably network overload, wait to send message
            sleep(Duration::from_secs(10)).await;
            println!("update_database_events panicked\n{:?}", err);
            telegram_bot::send_message("Carmine API `update_database_events` just panicked").await;
        } else {
            println!("Database updated with events");
        }
        sleep(Duration::from_secs(UPDATE_DELAY)).await;
        if let Err(err) = actix_web::rt::spawn(async { update_database_amm_state().await }).await {
            // failed, probably network overload, wait to send message
            sleep(Duration::from_secs(10)).await;
            println!("Update database amm state panicked\n{:?}", err);
            telegram_bot::send_message("Carmine API `update_database_amm_state` just panicked")
                .await;
        } else {
            println!("Database updated with AMM state");
        }
        sleep(Duration::from_secs(UPDATE_DELAY)).await;
    }
}
