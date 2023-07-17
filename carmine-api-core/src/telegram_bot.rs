use lazy_static::lazy_static;
use std::env::var;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

lazy_static! {
    static ref BOT_TOKEN_ENVAR: String = var("BOT_TOKEN").expect("Failed to read BOT_TOKEN");
    static ref CHAT_ID_ENVAR: String = var("CHAT_ID").expect("Failed to read CHAT_ID");
}

pub async fn send_message(msg: &str) {
    let bot_token = BOT_TOKEN_ENVAR.to_owned();
    let chat_id = CHAT_ID_ENVAR.to_owned();

    println!("sending message with {}{}", bot_token, chat_id);
    // Create a teloxide runtime and bot instance
    let bot = Bot::new(bot_token);

    // Send a message to the channel
    let res = bot
        .send_message(chat_id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .send()
        .await;
    println!("Telegram message result:\n{:?}", res);
}
