use std::env;

use carmine_api_core::telegram_bot;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = env::var("BOT_TOKEN").unwrap();
    let chat_id = env::var("CHAT_ID").unwrap();

    println!("bot_token: {}, chat_id: {}", bot_token, chat_id);

    let _ = telegram_bot::send_message("Test message sent by Carmine API").await;
}
