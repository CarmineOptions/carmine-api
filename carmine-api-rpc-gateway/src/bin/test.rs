use std::time::{Duration, Instant};

use carmine_api_core::network::{amm_address, call_lp_address, Network};
use carmine_api_rpc_gateway::{rpc_call, BlockTag, Entrypoint, RpcNode};
use dotenvy::dotenv;
use tokio::time::sleep;

#[allow(dead_code)]
#[derive(Debug)]
struct BenchResult {
    node: RpcNode,
    cumulative_time: u128,
    successful: usize,
    failed: usize,
    average: u128,
}

impl BenchResult {
    pub fn report(&self) {
        println!("{:#?}", self);
    }
}

async fn bench(node: RpcNode, number_of_runs: usize) -> BenchResult {
    let mut failed: usize = 0;
    let mut successful: usize = 0;
    let mut cum_time: u128 = 0;

    for _ in 0..number_of_runs {
        let before = Instant::now();

        let res = rpc_call(
            amm_address(&Network::Mainnet).to_string(),
            format!("{}", Entrypoint::GetAllNonExpiredOptionsWithPremia),
            vec![call_lp_address(&Network::Mainnet).to_owned()],
            BlockTag::Latest,
            node,
        )
        .await;
        let t = before.elapsed().as_millis();
        match res {
            Ok(_) => {
                successful += 1;
                cum_time += t;
            }
            Err(_) => {
                failed += 1;
            }
        };
    }

    BenchResult {
        node,
        cumulative_time: cum_time,
        successful,
        failed,
        average: cum_time / successful as u128,
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    bench(RpcNode::BlastAPI, 20).await.report();
    sleep(Duration::from_secs(10)).await;
    bench(RpcNode::Infura, 20).await.report();
    sleep(Duration::from_secs(10)).await;
    bench(RpcNode::CarmineJunoNode, 20).await.report();
}
