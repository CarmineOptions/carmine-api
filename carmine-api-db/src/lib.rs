use carmine_api_core::network::Network;
use carmine_api_core::schema::{self};
use carmine_api_core::types::{
    DbBlock, Event, IOption, OptionVolatility, OptionWithVolatility, Pool, PoolState,
    PoolStateWithTimestamp, Volatility,
};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

const BATCH_SIZE: usize = 100;

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
            .expect("Error saving batch of events");
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

pub fn get_pools(network: &Network) -> Vec<Pool> {
    use crate::schema::pools::dsl::*;

    let connection = &mut establish_connection(network);
    pools.load::<Pool>(connection).expect("Error loading pools")
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

pub fn get_options(network: &Network) -> Vec<IOption> {
    use crate::schema::options::dsl::*;

    let connection = &mut establish_connection(network);
    options
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
    use crate::schema::blocks::dsl::*;
    use crate::schema::pool_state::dsl::*;

    let connection = &mut establish_connection(network);
    pool_state
        .inner_join(blocks)
        .filter(lp_address.eq(pool_address))
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
        .collect()
}

pub fn get_options_volatility(network: &Network) -> Vec<OptionWithVolatility> {
    use crate::schema::blocks::dsl::*;
    use crate::schema::options::dsl::*;
    use crate::schema::options_volatility::dsl::*;

    let connection = &mut establish_connection(network);

    let all_options: Vec<IOption> = options
        .select(IOption::as_select())
        .load(connection)
        .expect("Failed getting all options");

    let volatilities: Vec<(OptionVolatility, DbBlock)> = options_volatility
        .inner_join(blocks)
        .select((OptionVolatility::as_select(), DbBlock::as_select()))
        .load::<(OptionVolatility, DbBlock)>(connection)
        .expect("Error loading option volatility");

    let mut options_with_volatilities: Vec<OptionWithVolatility> = vec![];

    for opt in all_options {
        // filter volatilities of the current option
        let option_specific_volatilities: Vec<Volatility> = volatilities
            .iter()
            .filter(|(vol, _)| opt.option_address == vol.option_address)
            .map(|(vol, block)| Volatility {
                block_number: block.block_number,
                timestamp: block.timestamp,
                volatility: vol.volatility.clone(),
                option_position: vol.option_position.clone(),
            })
            .collect();

        let mut last_value: Option<String> = None;

        let unique_volatilities: Vec<Volatility> =
            option_specific_volatilities
                .into_iter()
                .fold(vec![], |mut acc, cur| {
                    if &last_value != &cur.volatility {
                        last_value = cur.volatility.clone();
                        acc.push(cur);
                    }
                    acc
                });

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
            volatilities: unique_volatilities,
        });
    }

    options_with_volatilities
}
