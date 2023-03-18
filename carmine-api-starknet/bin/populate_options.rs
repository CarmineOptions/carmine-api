use carmine_api_db::{
    create_batch_of_options, get_option_addresses_from_events, models::NewIOption,
};
use carmine_api_starknet::Carmine;

#[tokio::main]
async fn main() {
    let option_addresses = get_option_addresses_from_events();
    let carmine = Carmine::new();
    let mut options: Vec<NewIOption> = Vec::new();
    let add_len = option_addresses.len();

    for address in option_addresses {
        let res = carmine.get_option_info_from_addresses(&address).await;

        if let Ok(option) = res {
            options.push(option);
        } else {
            println!("Failed to get option! {}", address);
        }
    }

    println!(
        "got {} addresses, fetched {} options",
        add_len,
        options.len()
    );
    create_batch_of_options(&options);
}
