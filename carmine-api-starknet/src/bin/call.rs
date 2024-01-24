use carmine_api_core::{network::Network, types::DbBlock};
use carmine_api_rpc_gateway::{rpc_call, BlockTag, RpcNode};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c = Carmine::new(Network::Mainnet);

    let res = c
        .get_amm_state(&DbBlock {
            block_number: 509916,
            timestamp: 1705597506,
        })
        .await;

    println!("{:#?}", res);
}
