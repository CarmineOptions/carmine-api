use amm_state::AmmStateObserver;
use carmine::Carmine;
use carmine_api_core::network::{Network, Protocol};
use starkscan::{get_events_from_starkscan, update_protocol_events};

pub mod amm_state;
pub mod carmine;
pub mod oracle;
pub mod starkscan;

pub async fn update_database() {
    let networks = vec![Network::Mainnet, Network::Testnet];
    for network in networks {
        let carmine = Carmine::new(network);
        carmine.get_options_with_addresses().await;
    }
    get_events_from_starkscan().await;
    update_protocol_events(&Protocol::CarmineOptions).await;
    update_protocol_events(&Protocol::Hashstack).await;
    update_protocol_events(&Protocol::ZkLend).await;
    update_protocol_events(&Protocol::ZETH).await;
    update_protocol_events(&Protocol::ZWBTC).await;
    update_protocol_events(&Protocol::ZUSDC).await;
    update_protocol_events(&Protocol::ZUSDT).await;
    update_protocol_events(&Protocol::ZDAI).await;
    AmmStateObserver::new().update_state().await;
}
