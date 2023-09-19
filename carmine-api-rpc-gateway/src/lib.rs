use std::env;

use carmine_api_core::{
    network::{amm_address, Network},
    types::DbBlock,
};
use lazy_static::lazy_static;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref BLAST_API_URL: String =
        env::var("BLAST_API_URL").expect("missing env var BLAST_API_URL");
    static ref INFURA_URL: String = env::var("INFURA_URL").expect("missing env var INFURA_URL");
    static ref INFURA_TESTNET_URL: String =
        env::var("INFURA_TESTNET_URL").expect("missing env var INFURA_TESTNET_URL");
    static ref CARMINE_JUNO_NODE_URL: String =
        env::var("CARMINE_JUNO_NODE_URL").expect("missing env var CARMINE_JUNO_NODE_URL");
}

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
    Number(i64),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Params {
    CallData(RpcCallData),
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
    #[serde(rename = "0x1600ab5a061ebfec75cb9a452efd442a99a0afeaa7c910b4083114f30bff2f1")]
    GetOptionInfoFromAddress,
    #[serde(rename = "0x14e79ebec158e1f661acf7d89ad12cd6cc4a47a712c3fbd62bc96bf65ca52f0")]
    GetOptionTokenAddress,
    #[serde(rename = "0x230b3b6ebadc35ebd0b91e93d39824daff6574cbe99bb7882037547cbb75197")]
    GetAllOptions,
    #[serde(rename = "0x3a59b17481476f4a9926cf55852dcc59e941e04e7c7afc16d1c887637e6b349")]
    GetAllLPTokenAddresses,
    #[serde(rename = "0xf58610cee3c804f0e87861ce266e465952f846d7f11a298b4a37f548065494")]
    GetPoolLockedCapital,
    #[serde(rename = "0x27e73afcf5eeea68f07ecec320a8a6ef66a0fec2a6555c98d7906efd26bafb9")]
    GetUnlockedCapital,
    #[serde(rename = "0x2b70e1b30215b8a9fdff94bce47077d43936e89d1180300a63f6b176b7d699e")]
    GetLpoolBalance,
    #[serde(rename = "0x399adda47235e1d39043a5931bead6042f3990866c6bd3091f582014f8a4f90")]
    GetValueOfPoolPosition,
    #[serde(rename = "0x68bb6b599048b94cdd7832f2ebbbda4b596b150896fc09bd70f88e2c488595")]
    GetUnderlyingForLptoken,
    #[serde(rename = "0xe8cc8c9fca554ee3ae877935823ca461ba94b34a427e3272fd465e0790e1af")]
    GetPoolVolatilityAuto,
    #[serde(rename = "0x2902df4b2064da30c68f1bfad76271da9c6b10a3cfc41396ae75eef960bfcb")]
    GetOptionPosition,
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RpcErrorResponse {
    code: u8,
    message: String,
    data: String,
}

#[derive(Debug, Deserialize)]
pub enum RpcError {
    ContractNotFound,
    ContractError(String),
    BlockNotFound,
    Other(String),
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcErrorResponse>,
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
        Params::CallData(RpcCallData {
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

fn rpc_request<T: Serialize>(body: T, node: RpcNode) -> RequestBuilder {
    let url: String = match node {
        RpcNode::BlastAPI => BLAST_API_URL.to_string(),
        RpcNode::Infura => INFURA_URL.to_string(),
        RpcNode::InfuraTestnet => INFURA_TESTNET_URL.to_string(),
        RpcNode::CarmineJunoNode => CARMINE_JUNO_NODE_URL.to_string(),
    };

    let client = reqwest::Client::new();

    client.post(url).json(&body)
}

pub async fn rpc_latest_block_number(node: RpcNode) -> Result<i64, RpcError> {
    let body = RpcCallBody {
        jsonrpc: "2.0".to_owned(),
        method: "starknet_blockNumber".to_owned(),
        id: 0,
        params: vec![],
    };
    let request = rpc_request(body, node);

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => {
            let msg = format!("Block number call failed: {:?}", e);
            return Err(RpcError::Other(msg));
        }
    };

    let parsed_response_option = match response.json::<Option<RpcResponse<i64>>>().await {
        Ok(res) => res,
        Err(e) => {
            println!("Request failed: {:?}", e);
            return Err(RpcError::Other("RPC block number failed".to_string()));
        }
    };

    let block_number = match parsed_response_option {
        Some(res) if res.result.is_some() => res.result.unwrap(),
        _ => {
            return Err(RpcError::Other(
                "Block number got empty response".to_string(),
            ));
        }
    };

    Ok(block_number)
}

pub async fn rpc_block_header(block: BlockTag, node: RpcNode) -> Result<DbBlock, RpcError> {
    let params = vec![Params::BlockTag(block)];
    let body = RpcCallBody {
        jsonrpc: "2.0".to_owned(),
        method: "starknet_getBlockWithTxHashes".to_owned(),
        id: 0,
        params,
    };
    let request = rpc_request(body, node);

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => {
            let msg = format!("Block header call failed: {:?}", e);
            return Err(RpcError::Other(msg));
        }
    };

    let parsed_response = response.json::<RpcResponse<DbBlock>>().await;

    let result = match parsed_response {
        Ok(res) => res.result,
        Err(e) => {
            println!("Request failed: {:?}", e);
            return Err(RpcError::Other("RPC block header failed".to_string()));
        }
    };

    match result {
        Some(block) => Ok(block),
        None => Err(RpcError::Other("RPC block header failed".to_string())),
    }
}

pub async fn rpc_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
    block: BlockTag,
    node: RpcNode,
) -> Result<Vec<String>, RpcError> {
    let body = build_call_body(contract, entry_point, calldata, block);

    let request = rpc_request(body, node);

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => {
            println!("call failed: {:#?}", e);
            return Err(RpcError::Other("call failed".to_string()));
        }
    };

    let parsed_response = response.json::<RpcResponse<Vec<String>>>().await;

    let rpc_response = match parsed_response {
        Ok(data) => data,
        Err(e) => {
            println!("Request failed: {:?}", e);
            return Err(RpcError::Other("RPC node request failed".to_string()));
        }
    };

    if let Some(e) = rpc_response.error {
        return match e.code {
            20 => Err(RpcError::ContractNotFound),
            24 => Err(RpcError::BlockNotFound),
            40 => Err(RpcError::ContractError(e.data)),
            _ => Err(RpcError::Other(format!(
                "RPC returned unexpected code {}",
                e.code
            ))),
        };
    }
    if let Some(data) = rpc_response.result {
        return Ok(data);
    }
    Err(RpcError::Other(format!(
        "Invalid result: {:?}",
        rpc_response
    )))
}

pub async fn blast_api_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, RpcError> {
    rpc_call(
        contract,
        entry_point,
        calldata,
        BlockTag::Latest,
        RpcNode::BlastAPI,
    )
    .await
}

pub async fn blast_api_latest_block_number() -> Result<i64, RpcError> {
    rpc_latest_block_number(RpcNode::BlastAPI).await
}

pub async fn infura_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, RpcError> {
    rpc_call(
        contract,
        entry_point,
        calldata,
        BlockTag::Latest,
        RpcNode::Infura,
    )
    .await
}

pub async fn infura_latest_block_number() -> Result<i64, RpcError> {
    rpc_latest_block_number(RpcNode::Infura).await
}

pub async fn carmine_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
    block: BlockTag,
) -> Result<Vec<String>, RpcError> {
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
) -> Result<Vec<String>, RpcError> {
    rpc_call(
        Contract::AMM,
        entry_point,
        calldata,
        block,
        RpcNode::CarmineJunoNode,
    )
    .await
}

pub async fn carmine_get_block_header(block: BlockTag) -> Result<DbBlock, RpcError> {
    rpc_block_header(block, RpcNode::CarmineJunoNode).await
}

pub async fn carmine_latest_block_number() -> Result<i64, RpcError> {
    rpc_latest_block_number(RpcNode::CarmineJunoNode).await
}
