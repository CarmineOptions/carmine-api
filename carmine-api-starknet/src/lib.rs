use amm_state::AmmStateObserver;
use carmine::Carmine;
use carmine_api_core::{
    network::{Network, Protocol},
    types::StarkScanEventSettled,
};
use carmine_api_db::create_batch_of_starkscan_events;
use starkscan::get_protocol_events;

pub mod amm_state;
pub mod carmine;
pub mod oracle;
pub mod starkscan;

pub async fn update_database_events() {
    let mut events: Vec<StarkScanEventSettled> = Vec::new();

    let protocols = [
        Protocol::CarmineOptions,
        Protocol::Hashstack,
        Protocol::ZkLend,
        Protocol::ZETH,
        Protocol::ZWBTC,
        Protocol::ZUSDC,
        Protocol::ZUSDT,
        Protocol::ZDAI,
    ];

    for protocol in protocols {
        // Call the get_protocol_events function for each protocol
        let current_events = get_protocol_events(&protocol).await;

        // Extend the combined_events vector with the events from the current protocol
        events.extend(current_events);
    }
    create_batch_of_starkscan_events(&events, &Network::Mainnet);
}

pub async fn update_database_amm_state() {
    let networks = vec![Network::Mainnet, Network::Testnet];
    for network in networks {
        let carmine = Carmine::new(network);
        carmine.get_options_with_addresses().await;
    }
    AmmStateObserver::new().update_state().await;
}
