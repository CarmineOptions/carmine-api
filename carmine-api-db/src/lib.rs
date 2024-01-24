use carmine_api_core::network::{protocol_address, Network, Protocol};
use carmine_api_core::schema::{self};
use carmine_api_core::types::{
    DbBlock, Event, IOption, NewReferralEvent, OptionVolatility, OptionWithVolatility, OraclePrice,
    Pool, PoolState, PoolStateWithTimestamp, ReferralCode, StarkScanEventSettled, Volatility,
};

use carmine_api_referral::referral_code::generate_referral_code;
use diesel::dsl::max;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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
        .filter(maturity.gt(1704495600)) // only get new AMM options
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
        .filter(block_number.lt(495000))
        .select((PoolState::as_select(), DbBlock::as_select()))
        .load::<(PoolState, DbBlock)>(connection)
        .expect("Error loading pool state")
        .into_iter()
        .map(|(pool, block)| PoolStateWithTimestamp {
            unlocked_cap: pool.unlocked_cap,
            locked_cap: pool.locked_cap,
            lp_balance: pool.lp_balance,
            pool_position: pool.pool_position,
            lp_token_value: pool.lp_token_value,
            lp_address: pool.lp_address,
            block_number: block.block_number,
            timestamp: block.timestamp,
        })
        .collect();

    data.sort_by(|a, b| b.block_number.cmp(&a.block_number));

    data
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
