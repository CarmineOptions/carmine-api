use carmine_api_core::{
    network::{Network, Protocol},
    pool::{get_all_pools, Pool},
    telegram_bot,
    types::{
        AppData, IOption, OraclePrice, OraclePriceConcise, PoolStateWithTimestamp,
        ReferralEventDigest, StarkScanEventSettled, TokenPair, TradeHistory, UserPoints, APY,
    },
};
use carmine_api_db::{
    get_all_user_points, get_options, get_options_volatility, get_oracle_prices, get_pool_state,
    get_protocol_events, get_protocol_events_from_block, get_referral_events,
    get_user_points_lastest_timestamp,
};
use carmine_api_starknet::carmine::Carmine;
use std::{collections::HashMap, time::SystemTime, vec};

mod apy;

// Only store Events we know and not ExpireOptionTokenForPool and Upgrade
const ALLOWED_METHODS: &'static [&'static str; 10] = &[
    "TradeOpen",
    "TradeClose",
    "TradeSettle",
    "DepositLiquidity",
    "WithdrawLiquidity",
    // Cairo1 contracts have this prefix
    "carmine_protocol::amm_core::amm::AMM::TradeOpen",
    "carmine_protocol::amm_core::amm::AMM::TradeClose",
    "carmine_protocol::amm_core::amm::AMM::TradeSettle",
    "carmine_protocol::amm_core::amm::AMM::DepositLiquidity",
    "carmine_protocol::amm_core::amm::AMM::WithdrawLiquidity",
];

pub struct Cache {
    network: Network,
    carmine: Carmine,
    events: Vec<StarkScanEventSettled>,
    options: HashMap<String, IOption>,
    all_non_expired: Vec<String>,
    trade_history: Vec<TradeHistory>,
    pools: Vec<Pool>,
    referrals: Vec<ReferralEventDigest>,
    user_points_timestamp: SystemTime,
    user_points: HashMap<String, UserPoints>,
    top_user_points: Vec<UserPoints>,
}

impl Cache {
    pub async fn new(network: Network) -> Self {
        let network = network;
        let carmine = Carmine::new(network);
        let events = get_protocol_events(&network, &Protocol::CarmineOptions);
        let options_vec = get_options(&network);
        let options = Cache::options_vec_to_hashmap(options_vec);
        let all_non_expired = vec![];
        let pools = get_all_pools(&network);
        let referrals = match network {
            Network::Mainnet => get_referral_events(),
            // no referral events on Testnet
            Network::Testnet => vec![],
        };
        let mut cache = Cache {
            network,
            carmine,
            events,
            options,
            all_non_expired,
            trade_history: Vec::new(),
            pools,
            referrals,
            user_points_timestamp: SystemTime::UNIX_EPOCH,
            user_points: HashMap::new(), // initialize empty
            top_user_points: vec![],     // initialize empty
        };

        cache.trade_history = Cache::generate_trade_history(&mut cache);
        cache.update_all_non_expired().await;
        cache.update_user_points();

        cache
    }

    pub fn get_app_data(&self) -> AppData {
        let all_non_expired = self.get_all_non_expired();
        let trade_history = self.get_trade_history();
        let option_volatility = get_options_volatility(&self.network);
        let state = self.generate_state_hashmap();
        let apy = self.generate_apy_hashmap();
        let oracle_prices = self.generate_oracle_prices_hash_map();
        let referrals = self.referrals.clone();
        let user_points = self.user_points.clone();
        let top_user_points = self.top_user_points.clone();

        AppData {
            all_non_expired,
            trade_history,
            option_volatility,
            state,
            apy,
            oracle_prices,
            referrals,
            user_points,
            top_user_points,
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

    fn generate_state_hashmap(&self) -> HashMap<String, Vec<PoolStateWithTimestamp>> {
        self.pools.iter().fold(HashMap::new(), |mut acc, pool| {
            acc.insert(
                pool.id.to_string(),
                get_pool_state(&pool.address, &self.network),
            );
            acc
        })
    }

    fn generate_apy_hashmap(&self) -> HashMap<String, APY> {
        self.pools.iter().fold(HashMap::new(), |mut acc, pool| {
            acc.insert(
                pool.id.to_string(),
                self.calculate_apy_for_pool(&pool.address),
            );
            acc
        })
    }

    fn update_user_points<'a>(&mut self) {
        if !matches!(self.network, Network::Mainnet) {
            // only do Mainnet
            return;
        }

        let timestamp_result = get_user_points_lastest_timestamp();

        let timestamp = match timestamp_result {
            Some(v) => v,
            _ => panic!("Failed getting UserPoints timestamp"),
        };

        if self.user_points_timestamp == timestamp {
            // no new points, keep the previous
            return;
        }

        // update timestamp for the next update cycle
        self.user_points_timestamp = timestamp;

        let mut user_points = get_all_user_points(timestamp);

        user_points.sort_by_key(|user_point| {
            -(user_point.trading_points + user_point.liquidity_points + user_point.referral_points)
        });

        let mut map = HashMap::new();
        let mut top = vec![];

        for user in user_points.iter().take(20) {
            let copy = UserPoints {
                address: user.address.to_string(),
                trading_points: user.trading_points,
                liquidity_points: user.liquidity_points,
                referral_points: user.referral_points,
            };
            top.push(copy);
        }

        for user in user_points {
            let address = user.address.to_string();
            map.insert(address, user);
        }

        self.user_points = map;
        self.top_user_points = top;
    }

    fn set_oracle_prices_pair(
        &self,
        prices_map: &mut HashMap<String, Vec<OraclePriceConcise>>,
        pair_id: String,
        prices: Vec<OraclePrice>,
    ) {
        let data = prices
            .into_iter()
            .filter(|oracle_price| &oracle_price.token_pair == &pair_id)
            .map(|full_price| OraclePriceConcise {
                price: full_price.price,
                decimals: full_price.decimals,
                last_updated_timestamp: full_price.last_updated_timestamp,
                block_number: full_price.block_number,
            })
            .collect();

        prices_map.insert(pair_id, data);
    }

    fn generate_oracle_prices_hash_map(&self) -> HashMap<String, Vec<OraclePriceConcise>> {
        let mut map: HashMap<String, Vec<OraclePriceConcise>> = HashMap::new();
        let oracle_prices = get_oracle_prices(&self.network);

        // TODO: optimize this
        self.set_oracle_prices_pair(&mut map, TokenPair::EthUsdc.id(), oracle_prices.clone());
        self.set_oracle_prices_pair(&mut map, TokenPair::BtcUsdc.id(), oracle_prices.clone());
        self.set_oracle_prices_pair(&mut map, TokenPair::StrkUsdc.id(), oracle_prices.clone());

        map
    }

    fn generate_trade_history(&self) -> Vec<TradeHistory> {
        let mut trade_history: Vec<TradeHistory> = self
            .events
            .iter()
            .filter(|event| ALLOWED_METHODS.contains(&event.key_name.as_str()))
            .map(|e| {
                let token_address = e.data[1].to_owned();
                let action = String::from(
                    &e.key_name
                        // remove prefix in C1 contracts
                        .replace("carmine_protocol::amm_core::amm::AMM::", ""),
                );
                let option = match self.options.get(&token_address) {
                    Some(v) => Some(v.clone()),
                    None => None,
                };
                let liquidity_pool: Option<String> = if action.as_str() == "DepositLiquidity"
                    || action.as_str() == "WithdrawLiquidity"
                {
                    let matched_pool = self
                        .pools
                        .iter()
                        .find(|&pool| pool.address == token_address.as_str());

                    match matched_pool {
                        Some(pool) => {
                            let pool_description = format!(
                                "{}/{} {}",
                                pool.base.symbol, pool.quote.symbol, pool.type_
                            );
                            Some(pool_description)
                        }
                        None => None,
                    }
                } else {
                    None
                };

                TradeHistory {
                    timestamp: e.timestamp,
                    action,
                    caller: e.data[0].to_owned(),
                    capital_transfered: e.data[2].to_owned(),
                    tokens_minted: e.data[4].to_owned(),
                    option,
                    liquidity_pool,
                }
            })
            .collect::<Vec<TradeHistory>>();

        trade_history.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        trade_history
    }

    fn calculate_apy_for_pool(&self, pool_address: &str) -> APY {
        let state = get_pool_state(pool_address, &self.network);
        apy::calculate_apy(&state)
    }

    pub fn update_options(&mut self) {
        let options_vec = get_options(&self.network);
        let options = Cache::options_vec_to_hashmap(options_vec);
        self.options = options;
    }

    pub fn update_events(&mut self) {
        let max_block_number_option = self.events.iter().max_by_key(|event| event.block_number);
        let max_block_number = match max_block_number_option {
            Some(event) => event.block_number,
            // did not find max block number, get all events
            None => {
                self.events = get_protocol_events(&self.network, &Protocol::CarmineOptions);
                return;
            }
        };
        let new_events = get_protocol_events_from_block(
            &self.network,
            &Protocol::CarmineOptions,
            max_block_number,
        );
        self.events.extend(new_events)
    }

    pub async fn update_all_non_expired(&mut self) {
        let new_non_expired_result = self.carmine.get_all_non_expired_options_with_premia().await;

        match new_non_expired_result {
            Ok(new_non_expired) => self.all_non_expired = new_non_expired,
            Err(e) => {
                println!(
                    "Failed getting non expired options: {:?}, \nNetwork {}",
                    e, &self.network
                );
                match &self.network {
                    Network::Mainnet => {
                        telegram_bot::send_message("Failed getting non expired options MAINNET")
                            .await
                    }
                    Network::Testnet => {
                        telegram_bot::send_message("Failed getting non expired options TESTNET")
                            .await
                    }
                }
            }
        }
    }

    pub fn update_trade_history(&mut self) {
        self.trade_history = Cache::generate_trade_history(self);
    }

    pub fn update_referral_events(&mut self) {
        self.referrals = match self.network {
            Network::Mainnet => get_referral_events(),
            // no referral events on Testnet
            Network::Testnet => vec![],
        };
    }

    pub async fn update(&mut self) {
        self.update_options();
        self.update_events();
        self.update_all_non_expired().await;
        self.update_trade_history();
        self.update_user_points();
    }
}
