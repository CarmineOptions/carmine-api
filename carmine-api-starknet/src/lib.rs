use amm_state::AmmStateObserver;
use carmine::Carmine;
use carmine_api_core::{
    network::{Network, Protocol},
    types::StarkScanEventSettled,
};
use carmine_api_db::create_batch_of_starkscan_events;
use starkscan::get_protocol_events;
use tokio::time::{sleep, Duration};

pub mod amm_state;
pub mod carmine;
pub mod oracle;
pub mod starkscan;

pub async fn update_database_events() {
    let mut events: Vec<StarkScanEventSettled> = Vec::new();

    let protocols = [
        Protocol::CarmineOptions,
        Protocol::CarmineGovernance,
        Protocol::CarminePoolEthUsdcCall,
        Protocol::CarminePoolEthUsdcPut,
        Protocol::CarminePoolBtcUsdcCall,
        Protocol::CarminePoolBtcUsdcPut,
        Protocol::CarminePoolEthStrkCall,
        Protocol::CarminePoolEthStrkPut,
        Protocol::CarminePoolStrkUsdcCall,
        Protocol::CarminePoolStrkUsdcPut,
        Protocol::ZkLendMarket,
        Protocol::ZkLendETH,
        Protocol::ZkLendUSDC,
        Protocol::ZkLendUSDT,
        Protocol::ZkLendWBTC,
        Protocol::ZkLendDAI,
        Protocol::NostraAlphaInterestModel,
        Protocol::NostraAlphaETH,
        Protocol::NostraAlphaETHCollateral,
        Protocol::NostraAlphaETHInterest,
        Protocol::NostraAlphaETHDebt,
        Protocol::NostraAlphaETHInterestCollateral,
        Protocol::NostraAlphaUSDC,
        Protocol::NostraAlphaUSDCCollateral,
        Protocol::NostraAlphaUSDCInterest,
        Protocol::NostraAlphaUSDCDebt,
        Protocol::NostraAlphaUSDCInterestCollateral,
        Protocol::NostraAlphaUSDT,
        Protocol::NostraAlphaUSDTCollateral,
        Protocol::NostraAlphaUSDTInterest,
        Protocol::NostraAlphaUSDTDebt,
        Protocol::NostraAlphaUSDTInterestCollateral,
        Protocol::NostraAlphaDAI,
        Protocol::NostraAlphaDAICollateral,
        Protocol::NostraAlphaDAIInterest,
        Protocol::NostraAlphaDAIDebt,
        Protocol::NostraAlphaDAIInterestCollateral,
        Protocol::NostraAlphaWBTC,
        Protocol::NostraAlphaWBTCCollateral,
        Protocol::NostraAlphaWBTCInterest,
        Protocol::NostraAlphaWBTCDebt,
        Protocol::NostraAlphaWBTCInterestCollateral,
        Protocol::NostraMainnetInterestModel,
        Protocol::Nostra2ETH,
        Protocol::Nostra2ETHCollateral,
        Protocol::Nostra2ETHInterest,
        Protocol::Nostra2ETHDebt,
        Protocol::Nostra2ETHInterestCollateral,
        Protocol::Nostra2USDC,
        Protocol::Nostra2USDCCollateral,
        Protocol::Nostra2USDCInterest,
        Protocol::Nostra2USDCDebt,
        Protocol::Nostra2USDCInterestCollateral,
        Protocol::Nostra2USDT,
        Protocol::Nostra2USDTCollateral,
        Protocol::Nostra2USDTInterest,
        Protocol::Nostra2USDTDebt,
        Protocol::Nostra2USDTInterestCollateral,
        Protocol::Nostra2DAI,
        Protocol::Nostra2DAICollateral,
        Protocol::Nostra2DAIInterest,
        Protocol::Nostra2DAIDebt,
        Protocol::Nostra2DAIInterestCollateral,
        Protocol::Nostra2WBTC,
        Protocol::Nostra2WBTCCollateral,
        Protocol::Nostra2WBTCInterest,
        Protocol::Nostra2WBTCDebt,
        Protocol::Nostra2WBTCInterestCollateral,
        Protocol::Nostra2WSTETH,
        Protocol::Nostra2WSTETHCollateral,
        Protocol::Nostra2WSTETHInterest,
        Protocol::Nostra2WSTETHDebt,
        Protocol::Nostra2WSTETHInterestCollateral,
        Protocol::Nostra2LORDS,
        Protocol::Nostra2LORDSCollateral,
        Protocol::Nostra2LORDSInterest,
        Protocol::Nostra2LORDSDebt,
        Protocol::Nostra2LORDSInterestCollateral,
        Protocol::Nostra2STRK,
        Protocol::Nostra2STRKCollateral,
        Protocol::Nostra2STRKInterest,
        Protocol::Nostra2STRKDebt,
        Protocol::Nostra2STRKInterestCollateral,
        Protocol::Hashstack,
        Protocol::Hashstack2,
        Protocol::HashstackBTCRToken,
        Protocol::HashstackBTCDToken,
        Protocol::HashstackETHRToken,
        Protocol::HashstackETHDToken,
        Protocol::HashstackUSDTRToken,
        Protocol::HashstackUSDTDToken,
        Protocol::HashstackUSDCRToken,
        Protocol::HashstackUSDCDToken,
        Protocol::HashstackDAIRToken,
        Protocol::HashstackDAIDToken,
        Protocol::HashstackStaking,
        Protocol::HashstackDiamond,
        Protocol::HashstackL3Diamond,
    ];

    for protocol in protocols {
        // Call the get_protocol_events function for each protocol
        let current_events = get_protocol_events(&Network::Mainnet, &protocol).await;
        println!("Fetched {} events for {}", current_events.len(), protocol);
        // Extend the combined_events vector with the events from the current protocol
        events.extend(current_events);

        // give DNS resolver time to cooldown
        sleep(Duration::from_secs(2)).await;
    }

    create_batch_of_starkscan_events(&events, &Network::Mainnet);
}

pub async fn update_database_amm_state(offset: i64) {
    let network = Network::Mainnet;
    let carmine = Carmine::new(network);
    carmine.get_options_with_addresses().await;
    AmmStateObserver::new().update_state(offset).await;
}

pub async fn plug_holes_amm_state() {
    AmmStateObserver::new().plug_holes_in_state().await;
}
