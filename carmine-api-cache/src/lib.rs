use carmine_api_core::{
    network::{Network, Protocol, LEGACY_AMM_CONTRACT_ADDRESS},
    pool::{get_all_pools, Pool},
    telegram_bot,
    types::{
        AppData, DefispringInfo, IOption, OraclePrice, OraclePriceConcise, PoolStateWithTimestamp,
        ReferralEventDigest, StarkScanEventSettled, TokenPair, TradeEvent, TradeHistory,
        UserPointsWithPosition, Vote, APY,
    },
    utils::{normalize_address, strike_from_hex},
};
use carmine_api_db::{
    get_all_user_points, get_braavos_users_proscore_80_with_timestamp, get_events_by_address,
    get_insurance_events, get_legacy_options, get_options, get_options_volatility,
    get_oracle_prices_since_new_amm, get_pool_state, get_protocol_events,
    get_protocol_events_from_block, get_referral_events, get_user_points_lastest_timestamp,
    get_votes,
};
use carmine_api_prices::HistoricalPrices;
use carmine_api_starknet::carmine::Carmine;
use defispring::get_defispring_stats;
use insurance_events::get_insurace_data;
use pail_events::transform_pail_events;
use std::{
    collections::HashMap,
    time::{Instant, SystemTime},
    vec,
};
use trade_data::get_trades;

mod apy;
pub mod defispring;
pub mod insurance_events;
pub mod pail_events;
pub mod trade_data;

// Only store Events we know and not ExpireOptionTokenForPool and Upgrade
const ALLOWED_METHODS: &'static [&'static str; 12] = &[
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
    // synthetic events
    "synthetic::DepositLiquidity",
    "synthetic::WithdrawLiquidity",
];

pub struct Cache {
    network: Network,
    carmine: Carmine,
    events: Vec<StarkScanEventSettled>,
    legacy_events: Vec<StarkScanEventSettled>,
    options: HashMap<String, IOption>,
    all_non_expired: Vec<String>,
    trade_history: Vec<TradeHistory>,
    legacy_trade_history: Vec<TradeHistory>,
    pools: Vec<Pool>,
    referrals: Vec<ReferralEventDigest>,
    user_points_timestamp: SystemTime,
    user_points: HashMap<String, UserPointsWithPosition>,
    top_user_points: Vec<UserPointsWithPosition>,
    defispring: DefispringInfo,
    oracle_prices: HashMap<String, Vec<OraclePriceConcise>>,
    historical_prices: HistoricalPrices,
}

impl Cache {
    pub async fn new(network: Network) -> Self {
        let network = network;
        let carmine = Carmine::new(network);
        let events = get_protocol_events(&network, &Protocol::CarmineOptions);
        let legacy_events = match network {
            Network::Mainnet => {
                get_events_by_address(&Network::Mainnet, LEGACY_AMM_CONTRACT_ADDRESS)
            }
            Network::Testnet => vec![],
        };

        let options_vec = get_options(&network);
        let options = Cache::options_vec_to_hashmap(options_vec);
        let all_non_expired = vec![];
        let pools = get_all_pools(&network);
        let referrals = match network {
            Network::Mainnet => get_referral_events(),
            // no referral events on Testnet
            Network::Testnet => vec![],
        };
        let defispring = match network {
            Network::Mainnet => get_defispring_stats()
                .await
                .expect("Failed getting defispring!"),
            // do not bother generating for testnet
            Network::Testnet => DefispringInfo {
                allocation: 0.0,
                apy: 0.0,
                tvl: 0.0,
            },
        };
        let oracle_prices = generate_oracle_prices_hash_map();
        let historical_prices = HistoricalPrices::new(&oracle_prices);

        let mut cache = Cache {
            network,
            carmine,
            events,
            legacy_events,
            options,
            all_non_expired,
            trade_history: vec![],
            legacy_trade_history: vec![],
            pools,
            referrals,
            user_points_timestamp: SystemTime::UNIX_EPOCH,
            user_points: HashMap::new(), // initialize empty
            top_user_points: vec![],     // initialize empty
            defispring,
            oracle_prices,
            historical_prices,
        };

        cache.trade_history = Cache::generate_trade_history(&mut cache);
        cache.legacy_trade_history = Cache::generate_legacy_trade_history(&mut cache);
        cache.update_all_non_expired().await;
        cache.update_user_points();

        cache
    }

    pub fn get_app_data(&self) -> AppData {
        let t0 = Instant::now();
        let all_non_expired = self.get_all_non_expired();
        println!("all non expired: {:?}", t0.elapsed());
        let trade_history = self.get_trade_history();
        println!("trade history: {:?}", t0.elapsed());
        let legacy_trade_history = self.get_legacy_trade_history();
        println!("legacy trade history: {:?}", t0.elapsed());
        let option_volatility = vec![];
        // let option_volatility = get_options_volatility(&self.network);
        println!("option volatility: {:?}", t0.elapsed());
        let state = self.generate_state_hashmap();
        println!("state: {:?}", t0.elapsed());
        let apy = self.generate_apy_hashmap();
        println!("apy: {:?}", t0.elapsed());
        let oracle_prices = self.oracle_prices.clone();
        println!("oracle prices: {:?}", t0.elapsed());
        let referrals = self.referrals.clone();
        println!("referrals: {:?}", t0.elapsed());
        let user_points = self.user_points.clone();
        println!("user points: {:?}", t0.elapsed());
        let top_user_points = self.top_user_points.clone();
        println!("top user points: {:?}", t0.elapsed());
        let trades = self.generate_trades_hashmap();
        println!("trades: {:?}", t0.elapsed());
        let votes = get_votes();
        println!("votes: {:?}", t0.elapsed());
        let mut votes_map: HashMap<String, Vec<Vote>> = HashMap::new();
        println!("votes map: {:?}", t0.elapsed());
        let trades_with_prices = get_trades(&trades, &self.historical_prices);
        println!("trades with prices: {:?}", t0.elapsed());
        let insurance_events = get_insurace_data(get_insurance_events(), &self.historical_prices);
        println!("insurance events: {:?}", t0.elapsed());

        for vote in votes.iter() {
            votes_map
                .entry(vote.user_address.clone())
                .or_insert_with(Vec::new)
                .push(vote.clone());
        }

        let defispring = self.defispring;
        println!("defispring: {:?}", t0.elapsed());
        let braavos_proscore = get_braavos_users_proscore_80_with_timestamp();
        println!("braavos proscore: {:?}", t0.elapsed());
        let pail_events = match &self.network {
            Network::Mainnet => {
                transform_pail_events(&get_protocol_events(&Network::Mainnet, &Protocol::Pail))
            }
            Network::Testnet => HashMap::new(),
        };
        println!("pail events: {:?}", t0.elapsed());

        AppData {
            all_non_expired,
            trade_history,
            legacy_trade_history,
            trades,
            option_volatility,
            state,
            apy,
            oracle_prices,
            referrals,
            user_points,
            top_user_points,
            votes,
            votes_map,
            defispring,
            braavos_proscore,
            trades_with_prices,
            insurance_events,
            pail_events,
        }
    }

    pub fn get_all_non_expired(&self) -> Vec<String> {
        self.all_non_expired.clone()
    }

    pub fn get_trade_history(&self) -> Vec<TradeHistory> {
        self.trade_history.clone()
    }

    pub fn get_legacy_trade_history(&self) -> Vec<TradeHistory> {
        self.legacy_trade_history.clone()
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

    fn generate_trades_hashmap(&self) -> HashMap<String, Vec<TradeEvent>> {
        let mut trades = vec![];
        for trade in self.trade_history.iter() {
            let option = match &trade.option {
                Some(o) => o,
                None => continue,
            };
            let strike_price = strike_from_hex(&option.strike_price);

            let trade_event = TradeEvent {
                timestamp: trade.timestamp,
                action: trade.action.to_string(),
                caller: trade.caller.to_string(),
                capital_transfered: trade.capital_transfered.to_string(),
                tokens_minted: trade.tokens_minted.to_string(),
                option_side: option.option_side,
                option_type: option.option_type,
                maturity: option.maturity,
                strike_price,
            };

            trades.push((option.lp_address.to_string(), trade_event));
        }

        let mut map = HashMap::new();
        for (lp_address, trade_event) in trades {
            if let Some(pool) = self.pools.iter().find(|p| p.address == lp_address) {
                // Use the pool id as the key in the HashMap
                map.entry(pool.id.to_string())
                    .or_insert_with(Vec::new)
                    .push(trade_event.clone());
            }
        }

        map
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

        let user_points = get_all_user_points(timestamp);

        let mut user_points_with_total: Vec<UserPointsWithPosition> = user_points
            .into_iter()
            .map(|u| UserPointsWithPosition {
                address: u.address,
                trading_points: u.trading_points,
                liquidity_points: u.liquidity_points,
                referral_points: u.referral_points,
                vote_points: u.vote_points,
                total_points: u.trading_points
                    + u.liquidity_points
                    + u.referral_points
                    + u.vote_points,
                position: 0, // temporary set to 0
            })
            .collect();

        user_points_with_total.sort_by_key(|u| -u.total_points); // negative for descending order

        let mut last_points = user_points_with_total
            .get(0)
            .expect("Zero user points")
            .total_points;
        let mut current_position = 1;

        let mut user_points_with_position = vec![];

        for mut user in user_points_with_total.into_iter() {
            if user.total_points < last_points {
                last_points = user.total_points;
                current_position += 1;
            }
            user.position = current_position;
            user_points_with_position.push(user);
        }

        let top: Vec<UserPointsWithPosition> = user_points_with_position[..20].to_vec();

        let mut map = HashMap::new();

        for user in user_points_with_position {
            let address = user.address.to_string();
            map.insert(address, user);
        }

        self.user_points = map;
        self.top_user_points = top;
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
                        .replace("carmine_protocol::amm_core::amm::AMM::", "")
                        .replace("synthetic::", ""),
                );
                let option = match self.options.get(&token_address) {
                    Some(v) => Some(v.clone()),
                    None => None,
                };
                let liquidity_pool: Option<String> = if action.as_str() == "DepositLiquidity"
                    || action.as_str() == "WithdrawLiquidity"
                {
                    self.pools
                        .iter()
                        .find(|&pool| pool.address == token_address.as_str())
                        .map(|p| p.descriptor())
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

    fn generate_legacy_trade_history(&self) -> Vec<TradeHistory> {
        let legacy_pool_hash_map = HashMap::from([
            (
                "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024",
                "ETH/USDC Call",
            ),
            (
                "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a",
                "ETH/USDC Put",
            ),
        ]);

        let legacy_options = get_legacy_options(&Network::Mainnet);

        let legacy_options_map =
            legacy_options
                .into_iter()
                .fold(HashMap::new(), |mut acc, option| {
                    acc.insert(option.option_address.clone(), option);
                    acc
                });

        let mut trade_history: Vec<TradeHistory> = self
            .legacy_events
            .iter()
            .filter(|event| ALLOWED_METHODS.contains(&event.key_name.as_str()))
            .map(|e| {
                let token_address = normalize_address(e.data[1].as_str());
                let action = String::from(
                    &e.key_name
                        // remove prefix in C1 contracts
                        .replace("carmine_protocol::amm_core::amm::AMM::", "")
                        .replace("synthetic::", ""),
                );
                let option = match legacy_options_map.get(&token_address) {
                    Some(v) => Some(v.clone()),
                    None => None,
                };
                let liquidity_pool: Option<String> = if action.as_str() == "DepositLiquidity"
                    || action.as_str() == "WithdrawLiquidity"
                {
                    legacy_pool_hash_map
                        .get(token_address.as_str())
                        .map(|&s| s.to_string())
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
                        // telegram_bot::send_message("Failed getting non expired options TESTNET")
                        //     .await
                    }
                }
            }
        }
    }

    pub async fn update_defispring(&mut self) {
        match get_defispring_stats().await {
            Ok(data) => self.defispring = data,
            Err(_) => telegram_bot::send_message("Failed updating DefiSpring data.").await,
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

    fn update_prices(&mut self) {
        if matches!(self.network, Network::Testnet) {
            return;
        }
        let oracle_prices = generate_oracle_prices_hash_map();
        let historical_prices = HistoricalPrices::new(&oracle_prices);

        self.oracle_prices = oracle_prices;
        self.historical_prices = historical_prices;
    }

    pub async fn update(&mut self) {
        let t0 = Instant::now();
        self.update_options();
        println!("Update options in: {}", t0.elapsed().as_secs());

        let t1 = Instant::now();
        self.update_events();
        println!("Update events in: {}", t1.elapsed().as_secs());

        let t2 = Instant::now();
        self.update_all_non_expired().await;
        println!("Update non expired in: {}", t2.elapsed().as_secs());

        let t3 = Instant::now();
        self.update_defispring().await;
        println!("Update defispring in: {}", t3.elapsed().as_secs());

        let t4 = Instant::now();
        self.update_trade_history();
        println!("Update trade history in: {}", t4.elapsed().as_secs());

        let t5 = Instant::now();
        self.update_user_points();
        println!("Update user points in: {}", t5.elapsed().as_secs());

        let t6 = Instant::now();
        self.update_prices();
        println!("Update prices in: {}", t6.elapsed().as_secs());
    }
}

fn set_oracle_prices_pair(
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

fn generate_oracle_prices_hash_map() -> HashMap<String, Vec<OraclePriceConcise>> {
    let mut map: HashMap<String, Vec<OraclePriceConcise>> = HashMap::new();
    let oracle_prices = get_oracle_prices_since_new_amm();

    // TODO: optimize this
    set_oracle_prices_pair(&mut map, TokenPair::EthUsdc.id(), oracle_prices.clone());
    set_oracle_prices_pair(&mut map, TokenPair::BtcUsdc.id(), oracle_prices.clone());
    set_oracle_prices_pair(&mut map, TokenPair::StrkUsdc.id(), oracle_prices.clone());
    set_oracle_prices_pair(&mut map, TokenPair::EkuboUsdc.id(), oracle_prices.clone());

    map
}
