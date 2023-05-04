use carmine_api_core::network::Network;
use carmine_api_core::schema::{self};
use carmine_api_core::types::{DbBlock, Event, IOption, OptionVolatility, Pool, PoolState};

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
            .expect("Error saving batch of volatilities");
    }
}
