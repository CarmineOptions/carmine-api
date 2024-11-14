use carmine_api_core::network::Network;
use carmine_api_rpc_gateway::BlockTag;
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c = Carmine::new(Network::Mainnet);

    let r = c.get_block_by_id(BlockTag::Latest).await;

    println!("{:#?}", r);
}
