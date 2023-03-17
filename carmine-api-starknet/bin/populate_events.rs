use carmine_api_db::create_batch_of_events;
use carmine_api_starknet::get_events_from_starkscan;

#[tokio::main]
async fn main() {
    let events = get_events_from_starkscan().await;
    create_batch_of_events(&events);

    println!("Events stored in the DB");
}
