use lazy_static::lazy_static;
use std::env::var;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

use crate::types::Messenger;

lazy_static! {
    static ref BOT_TOKEN_ENVAR: String = var("BOT_TOKEN").expect("Failed to read BOT_TOKEN");
    static ref CHAT_ID_ENVAR: String = var("CHAT_ID").expect("Failed to read CHAT_ID");
}

fn escape_markdown_v2(text: &str) -> String {
    let reserved_chars = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    text.chars()
        .map(|c| {
            if reserved_chars.contains(&c) {
                format!("\\{}", c) // Escape with a backslash
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub struct TelegramBot {
    chat_id: String,
    bot: teloxide::Bot,
}

impl TelegramBot {
    pub fn new() -> Self {
        let token = BOT_TOKEN_ENVAR.to_owned();
        let chat_id = CHAT_ID_ENVAR.to_owned();
        let bot = Bot::new(token);
        Self { chat_id, bot }
    }

    pub async fn send_message_async(self: Arc<Self>, msg: &str) {
        let escaped_message = escape_markdown_v2(msg);
        let res = self
            .bot
            .send_message(self.chat_id.clone(), escaped_message)
            .parse_mode(ParseMode::MarkdownV2)
            .send()
            .await;

        println!("Telegram message result:\n{:?}", res);
    }
}

impl Messenger for Arc<TelegramBot> {
    fn send_message(&self, msg: &str) {
        let msg_owned = msg.to_owned();
        let bot_clone = Arc::clone(self);

        tokio::spawn(async move {
            bot_clone.send_message_async(msg_owned.as_str()).await;
        });
    }
}

// TODO: this is the old implementation, should be replaced by struct in refactor
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
