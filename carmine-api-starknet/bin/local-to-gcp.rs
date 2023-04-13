use std::env;

use carmine_api_core::network::Network;
use carmine_api_core::schema;
use carmine_api_core::types::{Event, IOption};
use carmine_api_db::{get_events, get_options};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

fn get_db_url(network: &Network) -> String {
    let username = env::var("DB_USER").expect("Could not read \"DB_USER\"");
    let password = env::var("DB_PASSWORD").expect("Could not read \"DB_PASSWORD\"");
    let ip = env::var("DB_IP").expect("Could not read \"DB_IP\"");

    let base = format!("postgres://{}:{}@{}", username, password, ip);

    println!("{}", base);

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

fn create_batch_of_events(new_events: &Vec<Event>, network: &Network) {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = new_events.chunks(100);

    for chunk in chunks {
        diesel::insert_into(events)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
    }
}

fn create_batch_of_options(new_options: &Vec<IOption>, network: &Network) {
    use crate::schema::options::dsl::*;

    let mut connection = establish_connection(network);

    let chunks = new_options.chunks(100);

    for chunk in chunks {
        diesel::insert_into(options)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
    }
}

fn transfer_data(n: &Network) {
    let local_events = get_events(n);
    let local_options = get_options(n);

    create_batch_of_events(&local_events, n);
    create_batch_of_options(&local_options, n);
}

fn main() {
    dotenv().ok();
    let networks = vec![Network::Testnet, Network::Mainnet];

    for n in networks.iter() {
        transfer_data(n);
    }
}
