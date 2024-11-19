use carmine_api_cache::pail_events::transform_pail_events;
use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::get_protocol_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pail_events =
        transform_pail_events(&get_protocol_events(&Network::Mainnet, &Protocol::Pail));

    println!("{:#?}", pail_events);
}
