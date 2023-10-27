use carmine_api_core::{
    network::{Network, Protocol},
    types::StarkScanEventSettled,
};
use carmine_api_db::create_batch_of_starkscan_events;

use carmine_api_starknet::starkscan::get_block_range_events;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = &Network::Mainnet;
    let protocols = vec![
        &Protocol::Nostra2ETH,
        &Protocol::Nostra2ETHCollateral,
        &Protocol::Nostra2ETHInterest,
        &Protocol::Nostra2ETHDebt,
        &Protocol::Nostra2ETHInterestCollateral,
        &Protocol::Nostra2USDC,
        &Protocol::Nostra2USDCCollateral,
        &Protocol::Nostra2USDCInterest,
        &Protocol::Nostra2USDCDebt,
        &Protocol::Nostra2USDCInterestCollateral,
        &Protocol::Nostra2USDT,
        &Protocol::Nostra2USDTCollateral,
        &Protocol::Nostra2USDTInterest,
        &Protocol::Nostra2USDTDebt,
        &Protocol::Nostra2USDTInterestCollateral,
        &Protocol::Nostra2DAI,
        &Protocol::Nostra2DAICollateral,
        &Protocol::Nostra2DAIInterest,
        &Protocol::Nostra2DAIDebt,
        &Protocol::Nostra2DAIInterestCollateral,
        &Protocol::Nostra2WBTC,
        &Protocol::Nostra2WBTCCollateral,
        &Protocol::Nostra2WBTCInterest,
        &Protocol::Nostra2WBTCDebt,
        &Protocol::Nostra2WBTCInterestCollateral,
    ];

    let start = 350000;
    let mut current;
    let increment = 2000;
    let max = 354000;

    let mut events: Vec<StarkScanEventSettled> = vec![];

    for protocol in protocols {
        current = start;

        while current < max {
            let new_events =
                get_block_range_events(protocol, network, current, current + increment).await;
            println!("{} fetched {} - {}", protocol, current, current + increment);
            current = current + increment;
            events.extend(new_events);
        }
    }

    create_batch_of_starkscan_events(&events, network);
}
