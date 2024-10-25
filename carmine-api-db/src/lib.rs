use carmine_api_core::network::{
    protocol_address, Network, Protocol, NEW_AMM_GENESIS_BLOCK_NUMBER, NEW_AMM_GENESIS_TIMESTAMP,
};
use carmine_api_core::pool::{
    MAINNET_BTC_USDC_CALL, MAINNET_BTC_USDC_PUT, MAINNET_ETH_STRK_CALL, MAINNET_ETH_STRK_PUT,
    MAINNET_ETH_USDC_CALL, MAINNET_ETH_USDC_PUT, MAINNET_STRK_USDC_CALL, MAINNET_STRK_USDC_PUT,
};
use carmine_api_core::schema::pool_state::lp_token_value_usd;
use carmine_api_core::schema::{self};
use carmine_api_core::types::{
    BraavosBonus, BraavosBonusValues, DbBlock, Event, IOption, InsuranceEvent,
    InsuranceEventQueryable, NewReferralEvent, OptionVolatility, OptionWithVolatility, OraclePrice,
    Pool, PoolState, PoolStatePriceUpdate, PoolStateWithTimestamp, PoolTvlInfo, ReferralCode,
    ReferralEvent, ReferralEventDigest, StarkScanEventSettled, TokenPair, UserPoints, UserPointsDb,
    Volatility, Vote,
};

use carmine_api_referral::referral_code::generate_referral_code;
use diesel::dsl::max;
use diesel::sql_types::{Array, Text};
use diesel::{insert_into, prelude::*, update};
use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod lp_value;

const BATCH_SIZE: usize = 500;

fn get_db_url(network: &Network) -> String {
    let environment = env::var("ENVIRONMENT").expect("Could not read \"ENVIRONMENT\"");
    // your local DB
    if environment.as_str() == "local" {
        return match network {
            Network::Testnet => "postgres://localhost/carmine-testnet".to_string(),
            Network::Mainnet => "postgres://localhost/carmine-mainnet".to_string(),
        };
    }
    let username = env::var("DB_USER").expect("Could not read \"DB_USER\"");
    let password = env::var("DB_PASSWORD").expect("Could not read \"DB_PASSWORD\"");
    let ip = env::var("DB_IP").expect("Could not read \"DB_IP\"");

    let base = format!("postgres://{}:{}@{}", username, password, ip);
    match network {
        Network::Testnet => format!("{}/carmine-testnet", base).to_string(),
        Network::Mainnet => format!("{}/carmine-mainnet", base).to_string(),
    }
}

fn establish_connection(network: &Network) -> PgConnection {
    let database_url = get_db_url(network);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_event(new_event: Event, network: &Network) {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection(network);

    diesel::insert_into(events)
        .values(&new_event)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving event");
}

pub fn create_batch_of_events(new_events: &Vec<Event>, network: &Network) {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = new_events.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(events)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
    }
}

pub fn create_batch_of_starkscan_events(events: &Vec<StarkScanEventSettled>, network: &Network) {
    use crate::schema::starkscan_events::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = events.chunks(BATCH_SIZE);

    let mut inserted: u32 = 0;

    for chunk in chunks {
        let res = diesel::insert_into(starkscan_events)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
        inserted += res as u32;
    }

    println!("Inserted {} Starkscan events", inserted);
}

pub fn create_option(option: IOption, network: &Network) {
    use crate::schema::options::dsl::*;

    let mut connection = establish_connection(network);

    diesel::insert_into(options)
        .values(&option)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving option");
}

pub fn create_batch_of_options(new_options: &Vec<IOption>, network: &Network) {
    use crate::schema::options::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = new_options.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(options)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of options");
    }
}

pub fn create_block(data: &DbBlock, network: &Network) {
    use crate::schema::blocks::dsl::*;

    let mut connection = establish_connection(network);

    diesel::insert_into(blocks)
        .values(data)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving block");
}

pub fn create_pools(data: Vec<Pool>, network: &Network) {
    use crate::schema::pools::dsl::*;

    let mut connection = establish_connection(network);

    diesel::insert_into(pools)
        .values(&data)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving pools");
}

pub fn create_oracle_price(data: &OraclePrice, network: &Network) {
    use crate::schema::oracle_prices::dsl::*;

    let mut connection = establish_connection(network);

    diesel::insert_into(oracle_prices)
        .values(data)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving oracle price");
}

pub fn get_last_block_for_protocol_event(network: &Network, protocol: &Protocol) -> Option<i64> {
    use crate::schema::starkscan_events::dsl::*;

    let connection = &mut establish_connection(network);

    starkscan_events
        .filter(from_address.eq(protocol_address(network, protocol)))
        .select(max(block_number))
        .first(connection)
        .expect("Error getting last block_number for protocol events")
}

pub fn get_last_timestamp_for_protocol_event(
    network: &Network,
    protocol: &Protocol,
) -> Option<i64> {
    use crate::schema::starkscan_events::dsl::*;

    let connection = &mut establish_connection(network);

    starkscan_events
        .filter(from_address.eq(protocol_address(network, protocol)))
        .select(max(timestamp))
        .first(connection)
        .expect("Error loading last timestamp for protocol event")
}

// TODO: move events (Carmine specific) to starkscan_events (general)
pub fn get_last_timestamp_carmine_event(network: &Network) -> Option<i64> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection(network);

    events
        .filter(from_address.eq(protocol_address(network, &Protocol::CarmineOptions)))
        .select(max(timestamp))
        .first(connection)
        .expect("Error loading last timestamp for protocol event")
}

pub fn get_oracle_prices(network: &Network) -> Vec<OraclePrice> {
    use crate::schema::oracle_prices::dsl::*;

    let connection = &mut establish_connection(network);
    oracle_prices
        .load::<OraclePrice>(connection)
        .expect("Error loading oracle prices")
}

pub fn get_oracle_prices_from_block(network: &Network, initial_block: i64) -> Vec<OraclePrice> {
    use crate::schema::oracle_prices::dsl::*;

    let connection = &mut establish_connection(network);
    oracle_prices
        .filter(block_number.ge(initial_block))
        .load::<OraclePrice>(connection)
        .expect("Error loading oracle prices")
}

pub fn get_oracle_prices_since_new_amm() -> Vec<OraclePrice> {
    get_oracle_prices_from_block(&Network::Mainnet, NEW_AMM_GENESIS_BLOCK_NUMBER)
}

pub fn get_pools(network: &Network) -> Vec<Pool> {
    use crate::schema::pools::dsl::*;

    // TODO: currently old AMM is still in the DB, but we no longer care
    let legacy_lp_addresses = vec![
        "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024",
        "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a",
    ];

    let connection = &mut establish_connection(network);
    pools
        .filter(lp_address.ne_all(legacy_lp_addresses))
        .load::<Pool>(connection)
        .expect("Error loading pools")
}

pub fn get_protocol_events(network: &Network, protocol: &Protocol) -> Vec<StarkScanEventSettled> {
    use crate::schema::starkscan_events::dsl::*;

    let address = protocol_address(network, protocol);

    let connection = &mut establish_connection(network);
    starkscan_events
        .filter(from_address.eq(address))
        .load::<StarkScanEventSettled>(connection)
        .expect("Error loading starkscan events")
}

pub fn get_events_by_address(network: &Network, address: &str) -> Vec<StarkScanEventSettled> {
    use crate::schema::starkscan_events::dsl::*;

    let connection = &mut establish_connection(network);
    starkscan_events
        .filter(from_address.eq(address))
        .load::<StarkScanEventSettled>(connection)
        .expect("Error loading starkscan events")
}

pub fn get_protocol_events_from_block(
    network: &Network,
    protocol: &Protocol,
    from_block_number: i64,
) -> Vec<StarkScanEventSettled> {
    use crate::schema::starkscan_events::dsl::*;

    let address = protocol_address(network, protocol);

    let connection = &mut establish_connection(network);
    starkscan_events
        .filter(block_number.gt(from_block_number))
        .filter(from_address.eq(address))
        .load::<StarkScanEventSettled>(connection)
        .expect("Error loading starkscan events")
}

pub fn get_events(network: &Network) -> Vec<Event> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection(network);
    events
        .load::<Event>(connection)
        .expect("Error loading events")
}

pub fn get_events_by_caller_address(address: &str, network: &Network) -> Vec<Event> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection(network);
    events
        .filter(caller.eq(address))
        .load::<Event>(connection)
        .expect("Error loading events by caller address")
}

pub fn get_option_with_address(
    network: &Network,
    in_option_side: i16,
    in_maturity: i64,
    in_strike_price: &String,
    in_lp_address: &String,
) -> Option<IOption> {
    use crate::schema::options::dsl::*;

    let connection = &mut establish_connection(network);
    let res: Option<IOption> = options
        .filter(lp_address.eq(in_lp_address))
        .filter(maturity.eq(in_maturity))
        .filter(strike_price.eq(in_strike_price))
        .filter(option_side.eq(in_option_side))
        .first::<IOption>(connection)
        .optional()
        .expect("Error loading options");

    res
}

pub fn get_options(network: &Network) -> Vec<IOption> {
    use crate::schema::options::dsl::*;

    let connection = &mut establish_connection(network);
    options
        .filter(maturity.gt(NEW_AMM_GENESIS_TIMESTAMP)) // only get new AMM options
        .load::<IOption>(connection)
        .expect("Error loading options")
}

pub fn get_non_expired_options(network: &Network, ts: i64) -> Vec<IOption> {
    use crate::schema::options::dsl::*;

    let connection = &mut establish_connection(network);
    options
        .filter(maturity.gt(ts)) // only get new AMM options
        .load::<IOption>(connection)
        .expect("Error loading options")
}

pub fn get_legacy_options(network: &Network) -> Vec<IOption> {
    use crate::schema::options::dsl::*;

    let connection = &mut establish_connection(network);
    options
        .filter(maturity.lt(NEW_AMM_GENESIS_TIMESTAMP)) // only get new AMM options
        .load::<IOption>(connection)
        .expect("Error loading options")
}

pub fn get_block_by_number(num: i64, network: &Network) -> Option<DbBlock> {
    use crate::schema::blocks::dsl::*;

    let connection = &mut establish_connection(network);
    match blocks.find(num).first(connection) {
        Ok(v) => Some(v),
        _ => None,
    }
}

pub fn get_blocks_greater_than(min: i64, network: &Network) -> Vec<DbBlock> {
    use crate::schema::blocks::dsl::*;

    let connection = &mut establish_connection(network);
    blocks
        .filter(block_number.gt(min))
        .load::<DbBlock>(connection)
        .expect("Failed getting blocks")
}

pub fn get_blocks_since_new_amm() -> Vec<DbBlock> {
    get_blocks_greater_than(NEW_AMM_GENESIS_BLOCK_NUMBER, &Network::Mainnet)
}

pub fn get_last_block_in_db(network: &Network) -> DbBlock {
    use crate::schema::blocks::dsl::*;

    let connection = &mut establish_connection(network);

    // TODO: this is the right way to do it, but Diesel has some weird problem with it
    // let res = blocks.select(max(block_number)).first(connection);
    // match res {
    //     Ok(v) => v,
    //     Err(_) => None,
    // }

    // get all and find max, because Diesel does not like the SQL solution ¯\_(ツ)_/¯
    if let Ok(all_blocks) = blocks.load::<DbBlock>(connection) {
        return all_blocks
            .into_iter()
            .max_by_key(|b| b.block_number)
            .expect("did not find last block in DB");
    }
    unreachable!("did not find last block in DB");
}

pub fn create_batch_of_volatilities(volatilities: &Vec<OptionVolatility>, network: &Network) {
    use crate::schema::options_volatility::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = volatilities.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(options_volatility)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of volatilities");
    }
}

pub fn update_batch_of_volatilities(new_volatilities: &Vec<OptionVolatility>, network: &Network) {
    use crate::schema::options_volatility::dsl::*;

    let mut connection = establish_connection(network);

    let mut updated_sum = 0;

    for new_volatility in new_volatilities {
        let res: usize = diesel::update(
            options_volatility.filter(
                option_address
                    .eq(&new_volatility.option_address)
                    .and(block_number.eq(&new_volatility.block_number)),
            ),
        )
        .set(new_volatility)
        .execute(&mut connection)
        .expect("Error updating volatilities");

        updated_sum += res;
    }

    println!("Updated volatilities: {}", updated_sum);
}

pub fn create_batch_of_pool_states(states: &Vec<PoolState>, network: &Network) {
    use crate::schema::pool_state::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = states.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(pool_state)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of pool states");
    }
}

pub fn get_pool_state(pool_address: &str, network: &Network) -> Vec<PoolStateWithTimestamp> {
    use crate::schema::blocks::dsl::blocks;
    use crate::schema::pool_state::dsl::*;

    let connection = &mut establish_connection(network);
    let mut data: Vec<PoolStateWithTimestamp> = pool_state
        .inner_join(blocks)
        .filter(lp_address.eq(pool_address))
        // endstate of old AMM
        .filter(block_number.gt(495000))
        .select((PoolState::as_select(), DbBlock::as_select()))
        .load::<(PoolState, DbBlock)>(connection)
        .expect("Error loading pool state")
        .into_iter()
        .map(
            |(pool, block): (PoolState, DbBlock)| PoolStateWithTimestamp {
                unlocked_cap: pool.unlocked_cap,
                locked_cap: pool.locked_cap,
                lp_balance: pool.lp_balance,
                pool_position: pool.pool_position,
                lp_token_value: pool.lp_token_value,
                lp_token_value_usd: pool.lp_token_value_usd,
                underlying_asset_price: pool.underlying_asset_price,
                lp_address: pool.lp_address,
                block_number: block.block_number,
                timestamp: block.timestamp,
            },
        )
        .collect();

    data.sort_by(|a, b| b.block_number.cmp(&a.block_number));

    data
}

pub fn get_pool_states_with_prices(
    pool_address: &str,
    network: &Network,
) -> Vec<(PoolState, Vec<OraclePrice>)> {
    use crate::schema::oracle_prices::dsl::{block_number as oracle_block_number, oracle_prices};
    use crate::schema::pool_state::dsl::{
        block_number as pool_state_block_number, lp_address, pool_state,
    };

    let connection = &mut establish_connection(network);

    let pool_states_with_blocks: Vec<PoolState> = pool_state
        .filter(lp_address.eq(pool_address))
        .filter(pool_state_block_number.gt(495000))
        .filter(lp_token_value_usd.is_null()) // only states that do not have price yet
        .load::<PoolState>(connection)
        .expect("Error loading pool state");

    let block_numbers: Vec<i64> = pool_states_with_blocks
        .iter()
        .map(|ps| ps.block_number)
        .collect();

    let prices: Vec<OraclePrice> = oracle_prices
        .filter(oracle_block_number.eq_any(&block_numbers))
        .load::<OraclePrice>(connection)
        .expect("Error loading oracle prices");

    let mut prices_map: std::collections::HashMap<i64, Vec<OraclePrice>> =
        std::collections::HashMap::new();
    for price in prices {
        prices_map
            .entry(price.block_number)
            .or_insert_with(Vec::new)
            .push(price);
    }

    let mut results: Vec<(PoolState, Vec<OraclePrice>)> = pool_states_with_blocks
        .into_iter()
        .map(|pool| {
            let block_prices = prices_map.remove(&pool.block_number).unwrap_or_default();
            (pool, block_prices)
        })
        .collect();

    results.sort_by(|a, b| b.0.block_number.cmp(&a.0.block_number));

    results
}

pub fn update_pool_state_asset_prices(
    pool_state_price_updates: Vec<PoolStatePriceUpdate>,
) -> Result<(), diesel::result::Error> {
    use self::schema::pool_state::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    for price_update in pool_state_price_updates {
        diesel::update(
            pool_state.filter(
                lp_address
                    .eq(price_update.lp_address)
                    .and(block_number.eq(price_update.block_number)),
            ),
        )
        .set((
            lp_token_value_usd.eq(price_update.lp_token_value_usd),
            underlying_asset_price.eq(price_update.underlying_asset_price),
        ))
        .execute(connection)?;
    }

    Ok(())
}

pub fn get_pool_state_block_numbers_in_range(
    start_block: i64,
    end_block: i64,
    network: &Network,
) -> Vec<i64> {
    use crate::schema::pool_state::dsl::*;

    let connection = &mut establish_connection(network);
    pool_state
        .select(block_number)
        .filter(
            block_number
                .gt(start_block - 1)
                .and(block_number.lt(end_block + 1)),
        )
        .order(block_number.asc())
        .load::<i64>(connection)
        .expect("Error loading pool_state")
}

pub fn get_pool_state_block_holes(start: i64, end: i64, network: &Network) -> Vec<i64> {
    let blocks = get_pool_state_block_numbers_in_range(start, end, network);

    let range_numbers: Vec<i64> = (start..=end).collect();

    let holes: Vec<i64> = range_numbers
        .iter()
        .filter(|&x| !blocks.contains(x))
        .cloned()
        .collect();

    holes
}

pub fn get_options_volatility(network: &Network) -> Vec<OptionWithVolatility> {
    use crate::schema::blocks::dsl::*;
    use crate::schema::options::dsl::*;
    use crate::schema::options_volatility::dsl::*;

    let connection = &mut establish_connection(network);

    let start = SystemTime::now();
    let timestamp_now = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let cutoff: i64 = timestamp_now as i64 - 172800;

    let live_options: Vec<IOption> = options
        .filter(maturity.gt(cutoff))
        .select(IOption::as_select())
        .load(connection)
        .expect("Failed getting all options");

    let mut options_with_volatilities: Vec<OptionWithVolatility> = vec![];

    for opt in live_options {
        let volatilities: Vec<Volatility> = options_volatility
            .filter(crate::schema::options_volatility::dsl::option_address.eq(&opt.option_address))
            .inner_join(blocks)
            .order(crate::schema::blocks::dsl::block_number.desc())
            .select((OptionVolatility::as_select(), DbBlock::as_select()))
            .load::<(OptionVolatility, DbBlock)>(connection)
            .expect("Error loading option volatility")
            .iter()
            .map(|(vol, block)| Volatility {
                block_number: block.block_number,
                timestamp: block.timestamp,
                volatility: vol.volatility.clone(),
                option_position: vol.option_position.clone(),
            })
            .collect();

        // push current option with volatility
        options_with_volatilities.push(OptionWithVolatility {
            option_side: opt.option_side,
            maturity: opt.maturity,
            strike_price: opt.strike_price,
            quote_token_address: opt.quote_token_address,
            base_token_address: opt.base_token_address,
            option_type: opt.option_type,
            option_address: opt.option_address,
            lp_address: opt.lp_address,
            volatilities,
        });
    }

    options_with_volatilities
}

pub fn update_option_volatility(
    network: &Network,
    block: i64,
    vol: Option<String>,
    pos: Option<String>,
    address: String,
) {
    use crate::schema::options_volatility::dsl::*;

    let mut connection = establish_connection(network);

    let res = diesel::update(options_volatility)
        .filter(block_number.eq(block))
        .filter(option_address.eq(address))
        .set((volatility.eq(vol), option_position.eq(pos)))
        .execute(&mut connection);

    if let Err(_e) = res {
        println!("FAILED! {}", block);
    }
}

pub fn update_token_value(block: i64, pool: String, new_value: String, network: &Network) {
    use crate::schema::pool_state::dsl::*;

    let connection = &mut establish_connection(network);

    let res = diesel::update(pool_state)
        .filter(lp_address.eq(pool))
        .filter(block_number.eq(block))
        .set(lp_token_value.eq(new_value))
        .execute(connection);

    match res {
        Ok(size) => println!(
            "SUCCESS: updated block {} - returned value: {}",
            block, size
        ),
        Err(e) => println!("FAIL: block {} failed: {:?}", block, e),
    }
}

pub fn create_referral_pair(referral_pair: ReferralCode) {
    use crate::schema::referral_codes::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let _ = diesel::insert_into(referral_codes)
        .values(&referral_pair)
        .execute(connection)
        .expect("Error loading pool_state");
}

fn is_referral_code_available(code: &str) -> QueryResult<bool> {
    use crate::schema::referral_codes::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let count = referral_codes
        .filter(referral_code.eq(code))
        .count()
        .get_result::<i64>(connection)?;

    Ok(count == 0)
}

pub fn get_referral_code(referrer: String) -> String {
    use crate::schema::referral_codes::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let res: QueryResult<String> = referral_codes
        .filter(wallet_address.eq(&referrer))
        .select(referral_code)
        .first(connection);

    if let Ok(code) = res {
        // referral code already in DB
        return code;
    }

    let new_referral_code = loop {
        let temp = generate_referral_code();

        if let Ok(is_available) = is_referral_code_available(&temp) {
            if is_available {
                break temp;
            }
        }
    };

    create_referral_pair(ReferralCode {
        wallet_address: referrer,
        referral_code: new_referral_code.clone(),
    });

    new_referral_code
}

pub fn create_referral_event(event: NewReferralEvent) -> Result<usize, diesel::result::Error> {
    use crate::schema::referral_events::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    diesel::insert_into(referral_events)
        .values(&event)
        .execute(connection)
}

pub fn get_referral_events() -> Vec<ReferralEventDigest> {
    use crate::schema::referral_codes;
    use crate::schema::referral_events;

    let connection = &mut establish_connection(&Network::Mainnet);

    referral_events::table
        .inner_join(
            referral_codes::table
                .on(referral_events::referral_code.eq(referral_codes::referral_code)),
        )
        .select((
            referral_codes::wallet_address,
            referral_events::referred_wallet_address,
            referral_events::referral_code,
            referral_events::timestamp,
        ))
        .load::<(String, String, String, SystemTime)>(connection)
        .map(|results| {
            results
                .into_iter()
                .map(
                    |(
                        referee_wallet_address,
                        referred_wallet_address,
                        referral_code,
                        timestamp,
                    )| ReferralEventDigest {
                        referred_wallet_address,
                        referee_wallet_address,
                        referral_code,
                        timestamp,
                    },
                )
                .collect()
        })
        .expect("Error loading referral events")
}

pub fn create_insurance_event(event: InsuranceEvent) -> Result<usize, diesel::result::Error> {
    use crate::schema::insurance_events::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    diesel::insert_into(insurance_events)
        .values(&event)
        .execute(connection)
}

pub fn get_insurance_events() -> Vec<InsuranceEventQueryable> {
    use crate::schema::insurance_events::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    insurance_events
        .load::<InsuranceEventQueryable>(connection)
        .expect("Failed getting insurance events")
}

pub fn get_user_points(address: &str) -> Option<UserPointsDb> {
    use crate::schema::user_points::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);
    let res: QueryResult<UserPointsDb> = user_points
        .filter(user_address.eq(address))
        .order(timestamp.desc())
        .first(connection);

    match res {
        Ok(points) => Some(points),
        Err(_) => None,
    }
}

pub fn get_user_points_lastest_timestamp() -> Option<SystemTime> {
    use crate::schema::user_points::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    user_points
        .select(max(timestamp))
        .first(connection)
        .expect("Failed getting user points timestamp")
}

pub fn get_all_user_points(ts: SystemTime) -> Vec<UserPoints> {
    use crate::schema::user_points::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let records = user_points
        .filter(timestamp.eq(ts))
        .load::<UserPointsDb>(connection)
        .expect("Failed getting user points");

    records
        .into_iter()
        .map(|v| UserPoints {
            address: v.user_address,
            trading_points: v.trading_points,
            liquidity_points: v.liquidity_points,
            referral_points: v.referral_points,
            vote_points: v.vote_points,
        })
        .collect()
}

pub fn get_braavos_eligible_user_addresses() -> Vec<String> {
    use crate::schema::user_points::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    // Subquery to find the maximum timestamp
    let max_timestamp_option = user_points
        .select(max(timestamp))
        .first::<Option<SystemTime>>(connection)
        .expect("Failed getting braavos eligible timestamp");

    if let Some(max_timestamp) = max_timestamp_option {
        user_points
            .filter(timestamp.eq(max_timestamp))
            .select(user_address)
            .distinct()
            .load::<String>(connection)
            .expect("Failed getting braavos eligible user addresses")
    } else {
        vec![]
    }
}

pub fn get_votes() -> Vec<Vote> {
    use crate::schema::starkscan_events::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let events: Vec<StarkScanEventSettled> = starkscan_events
        .filter(key_name.eq_any(vec!["Voted", "governance::contract::Governance::Voted"]))
        .load::<StarkScanEventSettled>(connection)
        .expect("Error getting votes");

    events
        .into_iter()
        .map(|event| {
            let (prop_id_str, user_address, opinion_str) = (
                event.data[0].as_str(),
                event.data[1].to_string(),
                event.data[2].as_str(),
            );

            // historically there are multiple options for "nay"
            // but only "0x1" for "yay"
            let opinion = match opinion_str {
                "0x1" => 1,
                _ => 0,
            };

            Vote {
                timestamp: event.timestamp,
                user_address,
                prop_id: usize::from_str_radix(&prop_id_str[2..], 16)
                    .expect("Failed parsing prop_id"),
                opinion,
            }
        })
        .collect()
}

pub fn get_tvl_info() -> QueryResult<Vec<PoolTvlInfo>> {
    use crate::schema::blocks::dsl as blocks_dsl;
    use crate::schema::options::dsl as options_dsl;
    use crate::schema::options_volatility::dsl as volatility_dsl;
    use crate::schema::pool_state::dsl as pool_state_dsl;

    let connection = &mut establish_connection(&Network::Mainnet);

    let (max_block_num, max_block_timestamp) = blocks_dsl::blocks
        .order(blocks_dsl::block_number.desc())
        .select((blocks_dsl::block_number, blocks_dsl::timestamp))
        .first::<(i64, i64)>(connection)?;

    let results = options_dsl::options
        .inner_join(volatility_dsl::options_volatility.on(options_dsl::option_address.eq(volatility_dsl::option_address)))
        .inner_join(blocks_dsl::blocks.on(volatility_dsl::block_number.eq(blocks_dsl::block_number)))
        .inner_join(pool_state_dsl::pool_state.on(blocks_dsl::block_number.eq(pool_state_dsl::block_number)
                                                  .and(options_dsl::lp_address.eq(pool_state_dsl::lp_address))))
        .filter(options_dsl::maturity.gt(max_block_timestamp))
        .filter(options_dsl::option_side.eq(1))
        .filter(volatility_dsl::block_number.eq(max_block_num))
        .select((
            blocks_dsl::block_number,
            blocks_dsl::timestamp,
            options_dsl::lp_address,
            diesel::dsl::sql::<Array<Text>>("ARRAY_AGG(option_position) OVER (PARTITION BY options.lp_address, pool_state.unlocked_cap, pool_state.locked_cap)"),
            pool_state_dsl::unlocked_cap,
            pool_state_dsl::locked_cap
        ))
        .distinct_on((blocks_dsl::block_number, options_dsl::lp_address))
        .load::<PoolTvlInfo>(connection)?;

    Ok(results)
}

pub fn get_pool_tvl_map() -> HashMap<String, u128> {
    let tvl = get_tvl_info().expect("Query failed");

    let dec_18: u128 = 1_000_000_000_000_000_000;
    let dec_8: u128 = 100_000_000;
    let dec_6: u128 = 1_000_000;

    let address_to_divisor = HashMap::from([
        (MAINNET_BTC_USDC_CALL.address, (dec_8, dec_8)),
        (MAINNET_BTC_USDC_PUT.address, (dec_6, dec_8)),
        (MAINNET_ETH_USDC_CALL.address, (dec_18, dec_18)),
        (MAINNET_ETH_USDC_PUT.address, (dec_6, dec_18)),
        (MAINNET_ETH_STRK_CALL.address, (dec_18, dec_18)),
        (MAINNET_ETH_STRK_PUT.address, (dec_18, dec_18)),
        (MAINNET_STRK_USDC_CALL.address, (dec_18, dec_18)),
        (MAINNET_STRK_USDC_PUT.address, (dec_6, dec_18)),
    ]);

    let mut pool_tvl_map = HashMap::new();

    for tvl_info in tvl {
        let lp_address = tvl_info.lp_address.clone();
        let mut tvl: u128 = 0;

        let ad_hoc_precission = 1000;

        let (pool_divisor, option_divisor) = address_to_divisor
            .get(&tvl_info.lp_address.as_str())
            .expect("Failed getting divisor");

        tvl += u128::from_str_radix(&tvl_info.locked_capital[2..], 16)
            .expect("Failed to parse locked_capital")
            * ad_hoc_precission
            / pool_divisor;
        tvl += u128::from_str_radix(&tvl_info.unlocked_capital[2..], 16)
            .expect("Failed to parse unlocked_capital")
            * ad_hoc_precission
            / pool_divisor;

        for hex in tvl_info.option_positions {
            tvl += u128::from_str_radix(&hex[2..], 16).expect("Failed to parse option position")
                * ad_hoc_precission
                / option_divisor;
        }

        pool_tvl_map.insert(lp_address, tvl);
    }

    pool_tvl_map
}

pub fn get_price_block_numbers(pair: &TokenPair, min_block: i64, max_block: i64) -> Vec<i64> {
    use crate::schema::oracle_prices::dsl::*;

    let token_id = pair.id();

    let connection = &mut establish_connection(&Network::Mainnet);

    oracle_prices
        .select(block_number)
        .filter(token_pair.eq(token_id))
        .filter(block_number.ge(min_block))
        .filter(block_number.le(max_block))
        .load::<i64>(connection)
        .expect("Error loading block numbers")
}

pub fn upsert_braavos_pro_score_80(address: &str, ts: i64) -> QueryResult<usize> {
    use crate::schema::braavos_bonus::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    // Check if the user exists
    let existing_user: Option<BraavosBonus> = braavos_bonus
        .filter(user_address.eq(address))
        .first::<BraavosBonus>(connection)
        .optional()?;

    match existing_user {
        Some(mut user) => {
            if user.pro_score_80.is_none() {
                user.pro_score_80 = Some(ts);
                update(braavos_bonus.find(address))
                    .set(&user)
                    .execute(connection)
            } else {
                Ok(0)
            }
        }
        None => insert_into(braavos_bonus)
            .values(&BraavosBonus {
                user_address: address.to_string(),
                pro_score_80: Some(ts),
                braavos_referral: None,
            })
            .execute(connection),
    }
}

pub fn upsert_braavos_referral(address: &str, ts: i64) -> QueryResult<usize> {
    use crate::schema::braavos_bonus::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    // Check if the user exists
    let existing_user: Option<BraavosBonus> = braavos_bonus
        .filter(user_address.eq(address))
        .first::<BraavosBonus>(connection)
        .optional()?;

    match existing_user {
        Some(mut user) => {
            if user.braavos_referral.is_none() {
                user.braavos_referral = Some(ts);
                update(braavos_bonus.find(address))
                    .set(&user)
                    .execute(connection)
            } else {
                Ok(0)
            }
        }
        None => insert_into(braavos_bonus)
            .values(&BraavosBonus {
                user_address: address.to_string(),
                pro_score_80: None,
                braavos_referral: Some(ts),
            })
            .execute(connection),
    }
}

pub fn get_braavos_users_proscore_80() -> Vec<String> {
    use crate::schema::braavos_bonus::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    braavos_bonus
        .filter(pro_score_80.is_not_null())
        .select(user_address)
        .load::<String>(connection)
        .expect("Error loading pro score users")
}

pub fn get_braavos_users_proscore_80_with_timestamp() -> HashMap<String, BraavosBonusValues> {
    use crate::schema::braavos_bonus::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let results = braavos_bonus
        .filter(
            pro_score_80
                .is_not_null()
                .or(braavos_referral.is_not_null()),
        )
        .select((user_address, pro_score_80, braavos_referral))
        .load::<BraavosBonus>(connection)
        .expect("Error loading pro score users");

    let mut map = HashMap::new();
    for bonus in results {
        map.insert(
            bonus.user_address,
            BraavosBonusValues {
                pro_score_80: bonus.pro_score_80,
                braavos_referral: bonus.braavos_referral,
            },
        );
    }

    map
}

pub fn get_first_braavos_referrals() -> Result<Vec<(i64, String)>, diesel::result::Error> {
    use crate::schema::referral_events::dsl::*;

    let connection = &mut establish_connection(&Network::Mainnet);

    let braavos_referral_events: Vec<ReferralEvent> = referral_events
        .filter(referral_code.eq("braavos-referral-bonus"))
        .load::<ReferralEvent>(connection)
        .expect("Failed getting braavos referral events");

    let tuples: Vec<(i64, String)> = braavos_referral_events
        .into_iter()
        .map(|e| {
            let ts = e
                .timestamp
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            (ts.as_secs() as i64, e.referred_wallet_address)
        })
        .collect();

    let mut map: HashMap<String, i64> = HashMap::new();

    for (ts, s) in tuples.into_iter() {
        map.entry(s)
            .and_modify(|e| {
                if *e > ts {
                    *e = ts
                }
            })
            .or_insert(ts);
    }

    let without_duplicates: Vec<(i64, String)> = map.into_iter().map(|(s, ts)| (ts, s)).collect();

    Ok(without_duplicates)
}
