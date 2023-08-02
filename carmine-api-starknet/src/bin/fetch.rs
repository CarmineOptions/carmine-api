use std::env;

use carmine_api_core::network::{Network, Protocol};
use carmine_api_db::create_batch_of_starkscan_events;
use carmine_api_starknet::starkscan::{get_block_range_events, get_protocol_events};

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let _zklend_genesis_block = 48660;
    let _hashstack_genesis_block = 21178;

    let first_block = 108500;
    let last_block = 109500;
    let mut cur_from = first_block;
    let increment = 50;

    let prots = vec![
        // Protocol::NostraETH,
        // Protocol::NostraETHCollateral,
        // Protocol::NostraETHInterest,
        // Protocol::NostraETHDebt,
        Protocol::NostraETHInterestCollateral,
        Protocol::NostraUSDC,
        Protocol::NostraUSDCCollateral,
        Protocol::NostraUSDCInterest,
        Protocol::NostraUSDCDebt,
        Protocol::NostraUSDCInterestCollateral,
        Protocol::NostraUSDT,
        Protocol::NostraUSDTCollateral,
        Protocol::NostraUSDTInterest,
        Protocol::NostraUSDTDebt,
        Protocol::NostraUSDTInterestCollateral,
        Protocol::NostraDAI,
        Protocol::NostraDAICollateral,
        Protocol::NostraDAIInterest,
        Protocol::NostraDAIDebt,
        Protocol::NostraDAIInterestCollateral,
        Protocol::NostraWBTC,
        Protocol::NostraWBTCCollateral,
        Protocol::NostraWBTCInterest,
        Protocol::NostraWBTCDebt,
        Protocol::NostraWBTCInterestCollateral,
    ];

    for prot in prots {
        // get new ones
        // let events = get_protocol_events(&prot).await;
        // get range, used for only up to a certain block
        let events = get_block_range_events(&prot, 0, 35000).await;
        create_batch_of_starkscan_events(&events, &Network::Mainnet);
        println!("{} done", prot);
    }

    println!("DONE")
}
