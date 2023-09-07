use std::time::Instant;

use carmine_api_core::network::{call_lp_address, Network};
use carmine_api_rpc_gateway::{blast_api_call, infura_call, Contract, Entrypoint};

async fn infura_bench() {
    let number_of_runs: usize = 20;
    let mut failed: usize = 0;
    let mut succeeded: usize = 0;
    let mut cum_time: u128 = 0;

    for _ in 0..number_of_runs {
        let before = Instant::now();
        let res = infura_call(
            Contract::AMM,
            Entrypoint::GetAllNonExpiredOptionsWithPremia,
            vec![call_lp_address(&Network::Mainnet).to_owned()],
        )
        .await;
        let t = before.elapsed().as_millis();
        match res {
            Ok(_) => {
                succeeded += 1;
                cum_time += t;
            }
            Err(_) => {
                failed += 1;
            }
        };
    }
    println!(
        "\nINFURA\nCumulative time: {}, succeded: {}, failed: {}\naverage: {}\n",
        cum_time,
        succeeded,
        failed,
        cum_time / succeeded as u128
    );
}

async fn blast_api_bench() {
    let number_of_runs: usize = 20;
    let mut failed: usize = 0;
    let mut succeeded: usize = 0;
    let mut cum_time: u128 = 0;

    for _ in 0..number_of_runs {
        let before = Instant::now();
        let res = blast_api_call(
            Contract::AMM,
            Entrypoint::GetAllNonExpiredOptionsWithPremia,
            vec![call_lp_address(&Network::Mainnet).to_owned()],
        )
        .await;
        let t = before.elapsed().as_millis();
        match res {
            Ok(_) => {
                succeeded += 1;
                cum_time += t;
            }
            Err(_) => {
                failed += 1;
            }
        };
    }
    println!(
        "\nBLAST API\nCumulative time: {}, succeded: {}, failed: {}\naverage: {}\n",
        cum_time,
        succeeded,
        failed,
        cum_time / succeeded as u128
    );
}

#[tokio::main]
async fn main() {
    blast_api_bench().await;
    infura_bench().await;
}
