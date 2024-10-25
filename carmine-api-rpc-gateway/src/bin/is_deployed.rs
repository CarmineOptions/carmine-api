use carmine_api_rpc_gateway::{is_contract_deployed, RpcNode};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let is_deployed_result = is_contract_deployed(
        RpcNode::CarmineJunoNode,
        "0x4a260577bccdc4912e5850b795986fddbeeb4e8e8db8f67ff0537fe8d231459",
        740000,
    )
    .await;

    if let Ok(is_deployed) = is_deployed_result {
        match is_deployed {
            true => println!("is deployed"),
            false => println!("is NOT deployed"),
        }
    }
}
