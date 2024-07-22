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
        Protocol::NostraMainnetETH,
        Protocol::NostraMainnetETHCollateral,
        Protocol::NostraMainnetETHInterest,
        Protocol::NostraMainnetETHDebt,
        Protocol::NostraMainnetETHInterestCollateral,
        Protocol::NostraMainnetUSDC,
        Protocol::NostraMainnetUSDCCollateral,
        Protocol::NostraMainnetUSDCInterest,
        Protocol::NostraMainnetUSDCDebt,
        Protocol::NostraMainnetUSDCInterestCollateral,
        Protocol::NostraMainnetUSDT,
        Protocol::NostraMainnetUSDTCollateral,
        Protocol::NostraMainnetUSDTInterest,
        Protocol::NostraMainnetUSDTDebt,
        Protocol::NostraMainnetUSDTInterestCollateral,
        Protocol::NostraMainnetDAI,
        Protocol::NostraMainnetDAICollateral,
        Protocol::NostraMainnetDAIInterest,
        Protocol::NostraMainnetDAIDebt,
        Protocol::NostraMainnetDAIInterestCollateral,
        Protocol::NostraMainnetWBTC,
        Protocol::NostraMainnetWBTCCollateral,
        Protocol::NostraMainnetWBTCInterest,
        Protocol::NostraMainnetWBTCDebt,
        Protocol::NostraMainnetWBTCInterestCollateral,
        Protocol::NostraMainnetWSTETH,
        Protocol::NostraMainnetWSTETHCollateral,
        Protocol::NostraMainnetWSTETHInterest,
        Protocol::NostraMainnetWSTETHDebt,
        Protocol::NostraMainnetWSTETHInterestCollateral,
        Protocol::NostraMainnetLORDS,
        Protocol::NostraMainnetLORDSCollateral,
        Protocol::NostraMainnetLORDSInterest,
        Protocol::NostraMainnetLORDSDebt,
        Protocol::NostraMainnetLORDSInterestCollateral,
        Protocol::NostraMainnetSTRK,
        Protocol::NostraMainnetSTRKCollateral,
        Protocol::NostraMainnetSTRKInterest,
        Protocol::NostraMainnetSTRKDebt,
        Protocol::NostraMainnetSTRKInterestCollateral,
        Protocol::NostraMainnetNSTSTRK,
        Protocol::NostraMainnetNSTSTRKCollateral,
        Protocol::NostraMainnetNSTSTRKInterest,
        Protocol::NostraMainnetNSTSTRKDebt,
        Protocol::NostraMainnetNSTSTRKInterestCollateral,
        Protocol::NostraMainnetUNO,
        Protocol::NostraMainnetUNOCollateral,
        Protocol::NostraMainnetUNOInterest,
        Protocol::NostraMainnetUNODebt,
        Protocol::NostraMainnetUNOInterestCollateral,
        Protocol::NostraMainnetNSTR,
        Protocol::NostraMainnetNSTRCollateral,
        Protocol::NostraMainnetNSTRInterest,
        Protocol::NostraMainnetNSTRDebt,
        Protocol::NostraMainnetNSTRInterestCollateral,
        Protocol::NostraMainnetDAIV2,
        Protocol::NostraMainnetDAIV2Interest,
        Protocol::NostraMainnetDAIV2Debt,
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
