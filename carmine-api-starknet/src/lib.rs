use amm_state::AmmStateObserver;
use carmine::Carmine;
use carmine_api_core::network::{Network, Protocol};
use starkscan::update_protocol_events;

pub mod amm_state;
pub mod carmine;
pub mod oracle;
pub mod starkscan;

pub async fn update_database_events() {
    update_protocol_events(&Protocol::CarmineOptions).await;
    update_protocol_events(&Protocol::Hashstack).await;
    update_protocol_events(&Protocol::ZkLend).await;
    update_protocol_events(&Protocol::ZETH).await;
    update_protocol_events(&Protocol::ZWBTC).await;
    update_protocol_events(&Protocol::ZUSDC).await;
    update_protocol_events(&Protocol::ZUSDT).await;
    update_protocol_events(&Protocol::ZDAI).await;
}

pub async fn update_database_amm_state() {
    let networks = vec![Network::Mainnet, Network::Testnet];
    for network in networks {
        let carmine = Carmine::new(network);
        carmine.get_options_with_addresses().await;
    }
    AmmStateObserver::new().update_state().await;
}
