pub mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::Event;

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
        .select(option_token)
        .distinct()
        .load::<String>(connection)
        .expect("Error loading posts")
}

pub fn show_events() {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection();
    let results = events
        .load::<Event>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for event in results {
        println!("-----------");
        println!("{}", event.action);
        println!("{}", event.timestamp);
        println!("{}", event.caller);
    }
}
