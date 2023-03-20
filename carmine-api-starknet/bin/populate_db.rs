use std::collections::HashSet;

use carmine_api_db::{
    create_batch_of_events, create_batch_of_options,
    models::{NewEvent, NewIOption},
};
use carmine_api_starknet::{get_events_from_starkscan, Carmine};

#[tokio::main]
async fn main() {
    let carmine = Carmine::new();
    let events = get_events_from_starkscan().await;

    // get unique option_addresses from events through HashSet
    // and then convert into Vec
    let mut address_set: HashSet<String> = HashSet::new();
    for event in events.iter() {
        address_set.insert(String::from(&event.option_address));
    }
    let option_addresses: Vec<String> = address_set.into_iter().collect();

    // Fetch options for addresses
    let mut options: Vec<NewIOption> = Vec::new();
    for address in option_addresses {
        let res = carmine.get_option_info_from_addresses(&address).await;

        if let Ok(option) = res {
            options.push(option);
        } else {
            println!("Failed to get option! {}", address);
        }
    }

    // Get addresses for which we have options and filter events for these addresses
    let valid_options: Vec<String> = options.iter().map(|v| v.option_address.clone()).collect();
    let valid_events: Vec<NewEvent> = events
        .into_iter()
        .filter(|event| valid_options.contains(&event.option_address))
        .collect();

    // Create options first because of FK constraint
    create_batch_of_options(&options);
    create_batch_of_events(&valid_events);
}
