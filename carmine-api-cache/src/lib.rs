use std::{collections::HashMap, vec};

use carmine_api_core::{
    network::{call_lp_address, put_lp_address, Network},
    types::{AppData, Event, IOption, OraclePrice, TokenPair, TradeHistory},
    utils::token_pair_id,
};
use carmine_api_db::{
    get_events, get_options, get_options_volatility, get_oracle_prices, get_pool_state,
};
use carmine_api_starknet::carmine::Carmine;

mod apy;

// Only store Events we know and not ExpireOptionTokenForPool and Upgrade
const ALLOWED_METHODS: &'static [&'static str; 5] = &[
    "TradeOpen",
    "TradeClose",
    "TradeSettle",
    "DepositLiquidity",
    "WithdrawLiquidity",
];

pub struct Cache {
    network: Network,
    carmine: Carmine,
    events: Vec<Event>,
    options: HashMap<String, IOption>,
    all_non_expired: Vec<String>,
    trade_history: Vec<TradeHistory>,
    call_pool_address: &'static str,
    put_pool_address: &'static str,
}

impl Cache {
    pub fn new(network: Network) -> Self {
        let network = network;
        let carmine = Carmine::new(network);
        let events = get_events(&network);
        let options_vec = get_options(&network);
        let options = Cache::options_vec_to_hashmap(options_vec);
        let all_non_expired = vec![];

        let mut cache = Cache {
            network,
            carmine,
            events,
            options,
            all_non_expired,
            trade_history: Vec::new(),
            call_pool_address: call_lp_address(&network),
            put_pool_address: put_lp_address(&network),
        };

        cache.trade_history = Cache::generate_trade_history(&cache);

        cache
    }

    pub fn get_app_data(&self) -> AppData {
        let all_non_expired = self.get_all_non_expired();
        let trade_history = self.get_trade_history();
        let option_volatility = get_options_volatility(&self.network);
        let state_eth_usdc_call = get_pool_state(self.call_pool_address, &self.network);
        let state_eth_usdc_put = get_pool_state(self.put_pool_address, &self.network);
        let apy_eth_usdc_call = self.calculate_apy_for_pool(&self.call_pool_address);
        let apy_eth_usdc_put = self.calculate_apy_for_pool(&self.put_pool_address);
        let oracle_prices = self.generate_oracle_prices_hash_map();
        AppData {
            all_non_expired,
            trade_history,
            option_volatility,
            state_eth_usdc_call,
            state_eth_usdc_put,
            apy_eth_usdc_call,
            apy_eth_usdc_put,
            oracle_prices,
        }
    }

    pub fn get_all_non_expired(&self) -> Vec<String> {
        self.all_non_expired.clone()
    }

    pub fn get_trade_history(&self) -> Vec<TradeHistory> {
        self.trade_history.clone()
    }

    fn options_vec_to_hashmap(vec: Vec<IOption>) -> HashMap<String, IOption> {
        vec.into_iter().fold(HashMap::new(), |mut acc, option| {
            acc.insert(option.option_address.clone(), option);
            acc
        })
    }

    fn set_oracle_prices_pair(
        &self,
        prices_map: &mut HashMap<String, Vec<OraclePrice>>,
        pair_id: String,
        prices: Vec<OraclePrice>,
    ) {
        let data = prices
            .into_iter()
            .filter(|oracle_price| &oracle_price.token_pair == &pair_id)
            .collect();

        println!("oracle_prices data: {:?}", data);

        prices_map.insert(pair_id, data);
        println!(
            "oracle_prices map inside single token pair: {:?}",
            prices_map
        );
    }

    fn generate_oracle_prices_hash_map(&self) -> HashMap<String, Vec<OraclePrice>> {
        let mut map: HashMap<String, Vec<OraclePrice>> = HashMap::new();
        let oracle_prices = get_oracle_prices(&self.network);

        println!("oracle_prices: {:?}", oracle_prices);

        self.set_oracle_prices_pair(
            &mut map,
            token_pair_id(&TokenPair::EthUsdc),
            oracle_prices.clone(),
        );

        println!("oracle_prices map: {:?}", map);

        map
    }

    fn generate_trade_history(&self) -> Vec<TradeHistory> {
        let mut arr: Vec<TradeHistory> = Vec::new();

        for event in &self.events {
            if !ALLOWED_METHODS
                .iter()
                .any(|&action| action == &*event.action)
            {
                continue;
            }

            let option = match self.options.get(&event.token_address) {
                Some(v) => Some(v.clone()),
                None => None,
            };

            let liquidity_pool = match event.action.as_str() {
                "DepositLiquidity" | "WithdrawLiquidity"
                    if event.token_address.as_str() == self.put_pool_address =>
                {
                    Some("Put".to_string())
                }
                "DepositLiquidity" | "WithdrawLiquidity"
                    if event.token_address.as_str() == self.call_pool_address =>
                {
                    Some("Call".to_string())
                }
                _ => None,
            };

            let trade_history = TradeHistory {
                timestamp: event.timestamp,
                action: String::from(&event.action),
                caller: String::from(&event.caller),
                capital_transfered: String::from(&event.capital_transfered),
                tokens_minted: String::from(&event.tokens_minted),
                option,
                liquidity_pool,
            };
            arr.push(trade_history);
        }
        // sort by timestamp in ascending order
        arr.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        arr
    }

    fn calculate_apy_for_pool(&self, pool_address: &str) -> f64 {
        let state = get_pool_state(pool_address, &self.network);
        apy::calculate_apy(&state)
    }

    pub fn update_options(&mut self) {
        let options_vec = get_options(&self.network);
        let options = Cache::options_vec_to_hashmap(options_vec);
        self.options = options;
    }

    pub fn update_events(&mut self) {
        self.events = get_events(&self.network);
    }

    pub async fn update_all_non_expired(&mut self) {
        let new_non_expired_result = self.carmine.get_all_non_expired_options_with_premia().await;
        if let Ok(new_non_expired) = new_non_expired_result {
            self.all_non_expired = new_non_expired;
        }
    }

    pub fn update_trade_history(&mut self) {
        self.trade_history = Cache::generate_trade_history(&self);
    }

    pub async fn update(&mut self) {
        self.update_options();
        self.update_events();
        self.update_all_non_expired().await;
        self.update_trade_history();
    }
}
