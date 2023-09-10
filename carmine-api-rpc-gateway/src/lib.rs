use carmine_api_core::{
    constants,
    network::{amm_address, Network},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RpcCallData {
    contract_address: &'static str,
    entry_point_selector: Entrypoint,
    calldata: Vec<String>,
}

#[derive(Debug, Serialize)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "block_number")]
    Number(u64),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Params {
    Data(RpcCallData),
    BlockTag(BlockTag),
}

#[derive(Debug, Serialize)]
pub struct RpcCallBody {
    jsonrpc: String,
    method: String,
    id: u32,
    params: Vec<Params>,
}

#[derive(Debug, Serialize)]
pub enum Entrypoint {
    #[serde(rename = "0x2b20b26ede4304b68503c401a342731579b75844e5696ee13e6286cd51a9621")]
    GetOptionWithPositionOfUser,
    #[serde(rename = "0x28465ebd72d95a0985251c1cbd769fd70bd499003d1ed138cc4263dcd4154a8")]
    GetAllNonExpiredOptionsWithPremia,
    #[serde(rename = "0x3dbcec84ecc7488ae5f857e7a396bd0db953174c6824154aa472341d1fc6f63")]
    GetUserPoolInfos,
    #[serde(rename = "0x2f38757c6884edf9bd154a4cc0f03e9532c951f013089950d0a03242ca0c266")]
    GetTotalPremia,
}

pub enum Contract {
    AMM,
}

pub enum RpcNode {
    BlastAPI,
    Infura,
    InfuraTestnet,
    CarmineJunoNode,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<String>,
    // Other fields in the JSON response, if any
}

pub fn map_contract_to_address(contract: Contract) -> &'static str {
    match contract {
        Contract::AMM => amm_address(&Network::Mainnet),
    }
}

pub fn build_call_body(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
    block: BlockTag,
) -> RpcCallBody {
    let params = vec![
        Params::Data(RpcCallData {
            contract_address: map_contract_to_address(contract),
            entry_point_selector: entry_point,
            calldata,
        }),
        Params::BlockTag(block),
    ];
    RpcCallBody {
        jsonrpc: "2.0".to_owned(),
        method: "starknet_call".to_owned(),
        id: 0,
        params,
    }
}

pub async fn rpc_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
    block: BlockTag,
    node: RpcNode,
) -> Result<Vec<String>, String> {
    let body = build_call_body(contract, entry_point, calldata, block);

    println!("BODY: {}", serde_json::to_string_pretty(&body).unwrap());

    let url = match node {
        RpcNode::BlastAPI => constants::BLAST_API_URL,
        RpcNode::Infura => constants::INFURA_URL,
        RpcNode::InfuraTestnet => constants::INFURA_TESTNET_URL,
        RpcNode::CarmineJunoNode => constants::CARMINE_JUNO_NODE_URL,
    };

    let client = reqwest::Client::new();

    let request = client.post(url).json(&body);

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => {
            println!("call failed: {:#?}", e);
            return Err("call failed".to_string());
        }
    };

    let parsed_response = response.json::<RpcResponse<Vec<String>>>().await;

    let rpc_response = match parsed_response {
        Ok(data) => data,
        Err(e) => {
            println!("Request failed: {:?}", e);
            return Err("RPC node request failed".to_owned());
        }
    };

    if let Some(e) = rpc_response.error {
        return Err(e);
    }
    if let Some(data) = rpc_response.result {
        return Ok(data);
    }
    Err(format!("Invalid result: {:?}", rpc_response))
}

pub async fn blast_api_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, String> {
    rpc_call(
        contract,
        entry_point,
        calldata,
        BlockTag::Latest,
        RpcNode::BlastAPI,
    )
    .await
}

pub async fn infura_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, String> {
    rpc_call(
        contract,
        entry_point,
        calldata,
        BlockTag::Latest,
        RpcNode::Infura,
    )
    .await
}

pub async fn carmine_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
    block: BlockTag,
) -> Result<Vec<String>, String> {
    rpc_call(
        contract,
        entry_point,
        calldata,
        block,
        RpcNode::CarmineJunoNode,
    )
    .await
}

pub async fn carmine_amm_call(
    entry_point: Entrypoint,
    calldata: Vec<String>,
    block: BlockTag,
) -> Result<Vec<String>, String> {
    rpc_call(
        Contract::AMM,
        entry_point,
        calldata,
        block,
        RpcNode::CarmineJunoNode,
    )
    .await
}
