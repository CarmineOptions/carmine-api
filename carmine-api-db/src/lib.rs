use carmine_api_core::network::Network;
use carmine_api_core::schema;
use carmine_api_core::types::{Event, IOption};

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
