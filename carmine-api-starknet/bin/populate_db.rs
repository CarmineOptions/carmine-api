use carmine_api_db::{create_batch_of_events, create_batch_of_options, models::NewEvent};
use carmine_api_starknet::{get_events_from_starkscan, Carmine};

#[tokio::main]
async fn main() {
    let carmine = Carmine::new();
    let options = carmine.get_options_with_addresses().await;
    let events = get_events_from_starkscan().await;
    let events_count = events.len();

    // Get addresses for which we have options and filter events for these addresses
    let available_option_addresses: Vec<String> =
        options.iter().map(|v| v.option_address.clone()).collect();
    let valid_events: Vec<NewEvent> = events
        .into_iter()
        .filter(|event| available_option_addresses.contains(&event.option_address))
        .collect();

    println!(
        "Got {} events, {} of them are valid",
        events_count,
        valid_events.len()
    );

    // Create options first because of FK constraint
    create_batch_of_options(&options);
    create_batch_of_events(&valid_events);
}
