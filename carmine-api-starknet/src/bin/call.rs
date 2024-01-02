use carmine_api_rpc_gateway::{rpc_call, BlockTag, RpcNode};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let res = rpc_call(
        String::from("0x076dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa"),
        String::from("0xe8cc8c9fca554ee3ae877935823ca461ba94b34a427e3272fd465e0790e1af"),
        vec![
            String::from("0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a"),
            String::from("0x6597467f"),
            String::from("0x1068000000000000000"),
        ],
        BlockTag::Latest,
        RpcNode::CarmineJunoNode,
    )
    .await;

    println!("{:#?}", res);
}
