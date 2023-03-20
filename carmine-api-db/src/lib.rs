pub mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::IOption;
use std::env;

use crate::models::{Event, TradeHistory};

const BATCH_SIZE: usize = 100;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_event(new_event: models::NewEvent) {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    diesel::insert_into(events)
        .values(&new_event)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving event");
}

pub fn create_batch_of_events(new_events: &Vec<models::NewEvent>) {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    let chunks = new_events.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(events)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
    }
}

pub fn create_option(option: models::NewIOption) {
    use crate::schema::options::dsl::*;

    let mut connection = establish_connection();

    diesel::insert_into(options)
        .values(&option)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving option");
}

pub fn create_batch_of_options(new_options: &Vec<models::NewIOption>) {
    use crate::schema::options::dsl::*;

    let mut connection = establish_connection();

    let chunks = new_options.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(options)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
    }
}

pub fn get_option_addresses_from_events() -> Vec<String> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection();
    events
        .select(option_address)
        .distinct()
        .load::<String>(connection)
        .expect("Error loading posts")
}

pub fn get_events() -> Vec<Event> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection();
    events
        .load::<Event>(connection)
        .expect("Error loading events")
}

pub fn get_events_by_caller_address(address: &str) -> Vec<Event> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection();
    events
        .filter(caller.eq(address))
        .load::<Event>(connection)
        .expect("Error loading events by caller address")
}

pub fn get_options() -> Vec<IOption> {
    use crate::schema::options::dsl::*;

    let connection = &mut establish_connection();
    options
        .load::<IOption>(connection)
        .expect("Error loading options")
}

pub fn get_trade_history() -> Vec<TradeHistory> {
    use crate::schema::events;
    use crate::schema::options;
    let connection = &mut establish_connection();

    let events_with_option: Vec<(IOption, Event)> = options::table
        .inner_join(events::table)
        .select((IOption::as_select(), Event::as_select()))
        .load::<(IOption, Event)>(connection)
        .expect("Failed to get options - events inner join");

    let mut trade_history_list: Vec<TradeHistory> = events_with_option
        .into_iter()
        .map(|(o, e)| TradeHistory {
            timestamp: e.timestamp,
            action: e.action,
            caller: e.caller,
            capital_transfered: e.capital_transfered,
            option_tokens_minted: e.option_tokens_minted,
            option_side: o.option_side,
            maturity: o.maturity,
            strike_price: o.strike_price,
            quote_token_address: o.quote_token_address,
            base_token_address: o.base_token_address,
            option_type: o.option_type,
        })
        .collect();

    trade_history_list.sort_by_key(|v| v.timestamp);

    trade_history_list
}
