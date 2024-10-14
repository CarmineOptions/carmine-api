use std::time::{Duration, Instant};

use carmine_api_core::network::{amm_address, Network};
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
    average: Option<u128>,
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
            vec![
                "0x70cad6be2c3fc48c745e4a4b70ef578d9c79b46ffac4cd93ec7b61f951c7c5c".to_owned(), // ETH USDC CALL Pool address
            ],
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

    let average = match successful {
        0 => None,
        _ => Some(cum_time / successful as u128),
    };

    BenchResult {
        node,
        cumulative_time: cum_time,
        successful,
        failed,
        average,
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
