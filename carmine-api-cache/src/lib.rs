use std::collections::HashMap;

use carmine_api_core::{Event, IOption, TradeHistory};
use carmine_api_starknet::{get_events_from_starkscan, get_new_events_from_starkscan, Carmine};

pub struct Cache {
    carmine: Carmine,
    events: Vec<Event>,
    options: HashMap<String, IOption>,
    all_non_expired: Vec<String>,
    trade_history: Vec<TradeHistory>,
}

impl Cache {
    pub async fn new() -> Self {
        let carmine = Carmine::new();
        let events = get_events_from_starkscan().await;
        let options_vec = carmine.get_options_with_addresses().await;
        let options = Cache::options_vec_to_hashmap(options_vec);
        let all_non_expired = carmine.get_all_non_expired_options_with_premia().await;

        let mut cache = Cache {
            carmine,
            events,
            options,
            all_non_expired,
            trade_history: Vec::new(),
        };

        let trade_history = Cache::generate_trade_history(&cache);

        cache.trade_history = trade_history;

        cache
    }

    pub fn get_all_non_expired(&self) -> Vec<String> {
        let mut arr: Vec<String> = Vec::new();

        for v in &self.all_non_expired {
            arr.push(String::from(v));
        }

        arr
    }

    pub fn get_trade_history(&self) -> Vec<TradeHistory> {
        let mut arr: Vec<TradeHistory> = Vec::new();

        for v in &self.trade_history {
            let copy = TradeHistory {
                timestamp: v.timestamp,
                action: String::from(&v.action),
                caller: String::from(&v.caller),
                capital_transfered: String::from(&v.capital_transfered),
                option_tokens_minted: String::from(&v.option_tokens_minted),
                option_side: v.option_side,
                maturity: v.maturity,
                strike_price: String::from(&v.strike_price),
                quote_token_address: String::from(&v.quote_token_address),
                base_token_address: String::from(&v.base_token_address),
                option_type: v.option_type,
            };

            arr.push(copy);
        }

        arr
    }

    fn options_vec_to_hashmap(vec: Vec<IOption>) -> HashMap<String, IOption> {
        let mut map: HashMap<String, IOption> = HashMap::new();

        for option in vec {
            let address = String::from(&option.option_address);
            map.insert(address, option);
        }

        map
    }

    fn generate_trade_history(&self) -> Vec<TradeHistory> {
        let mut arr: Vec<TradeHistory> = Vec::new();

        for event in &self.events {
            let option = match self.options.get(&event.option_address) {
                Some(v) => v,
                None => {
                    // no option for this event
                    continue;
                }
            };

            let trade_history = TradeHistory {
                timestamp: event.timestamp,
                action: String::from(&event.action),
                caller: String::from(&event.caller),
                capital_transfered: String::from(&event.capital_transfered),
                option_tokens_minted: String::from(&event.option_tokens_minted),
                option_side: option.option_side,
                maturity: option.maturity,
                strike_price: String::from(&option.strike_price),
                quote_token_address: String::from(&option.quote_token_address),
                base_token_address: String::from(&option.base_token_address),
                option_type: option.option_type,
            };

            arr.push(trade_history);
        }

        arr
    }

    pub async fn update_options(&mut self) {
        let options_vec = self.carmine.get_options_with_addresses().await;
        self.options = Cache::options_vec_to_hashmap(options_vec);
    }

    pub async fn update_events(&mut self) {
        let new_events = get_new_events_from_starkscan(&self.events).await;
        let mut unique_new_events = new_events
            .into_iter()
            .filter(|e| !self.events.contains(e))
            .collect();
        self.events.append(&mut unique_new_events);
    }

    pub async fn update_all_non_expired(&mut self) {
        self.all_non_expired = self.carmine.get_all_non_expired_options_with_premia().await;
    }

    pub fn update_trade_history(&mut self) {
        self.trade_history = Cache::generate_trade_history(&self);
    }

    pub async fn update(&mut self) {
        self.update_options().await;
        self.update_events().await;
        self.update_all_non_expired().await;
        // update_trade_history is synchronous
        self.update_trade_history();
    }
}
