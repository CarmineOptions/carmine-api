use carmine_api_rpc_gateway::{
    blast_api_latest_block_number, carmine_latest_block_number, infura_latest_block_number,
    rpc_call, BlockTag, RpcNode,
};
use dotenvy::dotenv;
use tokio::try_join;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let latest_blocks = try_join!(
        carmine_latest_block_number(),
        infura_latest_block_number(),
        blast_api_latest_block_number(),
    );

    println!("{:#?}", latest_blocks);

    let ten_to_18: String = format!("{:#01x}", 10_u128.pow(18));

    let res = rpc_call(
        "0x076dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa".to_string(),
        "0x68bb6b599048b94cdd7832f2ebbbda4b596b150896fc09bd70f88e2c488595".to_string(),
        vec![
            "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024".to_string(),
            ten_to_18,
            "0x0".to_string(),
        ],
        BlockTag::Latest,
        RpcNode::CarmineJunoNode,
    )
    .await;

    println!("{:#?}", res)
}
