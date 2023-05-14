use amm_state::AmmStateObserver;
use carmine::Carmine;
use carmine_api_core::network::Network;
use carmine_api_db::get_events;
use starkscan::get_new_events_from_starkscan;

pub mod amm_state;
pub mod carmine;
pub mod starkscan;

pub async fn update_database() {
    let networks = vec![Network::Mainnet, Network::Testnet];
    for network in networks {
        let carmine = Carmine::new(network);
        carmine.get_options_with_addresses().await;
        get_new_events_from_starkscan(&get_events(&network), &network).await;
    }
    AmmStateObserver::new().update_state().await;
}
