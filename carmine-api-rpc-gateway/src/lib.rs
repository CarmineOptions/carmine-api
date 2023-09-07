use carmine_api_core::network::{amm_address, Network};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use starknet::{core::types::FieldElement, macros::selector};

#[derive(Debug, Serialize)]
pub struct RpcCallData {
    contract_address: &'static str,
    entry_point_selector: String,
    calldata: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Params {
    Data(RpcCallData),
    Block(String),
}

#[derive(Debug, Serialize)]
pub struct RpcCallBody {
    jsonrpc: String,
    method: String,
    id: u32,
    params: Vec<Params>,
}

pub enum Entrypoint {
    GetOptionWithPositionOfUser,
    GetAllNonExpiredOptionsWithPremia,
    GetUserPoolInfos,
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

fn to_hex(v: FieldElement) -> String {
    format!("{:#x}", v)
}

pub fn map_entrypoint_to_selector(entry_point: Entrypoint) -> String {
    let felt = match entry_point {
        Entrypoint::GetOptionWithPositionOfUser => {
            selector!("get_option_with_position_of_user")
        }
        Entrypoint::GetAllNonExpiredOptionsWithPremia => {
            selector!("get_all_non_expired_options_with_premia")
        }
        Entrypoint::GetUserPoolInfos => {
            selector!("get_user_pool_infos")
        }
        Entrypoint::GetTotalPremia => {
            selector!("get_total_premia")
        }
    };
    to_hex(felt)
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
) -> RpcCallBody {
    let params = vec![
        Params::Data(RpcCallData {
            contract_address: map_contract_to_address(contract),
            entry_point_selector: map_entrypoint_to_selector(entry_point),
            calldata,
        }),
        Params::Block("latest".to_owned()),
    ];
    RpcCallBody {
        jsonrpc: "2.0".to_owned(),
        method: "starknet_call".to_owned(),
        id: 0,
        params,
    }
}

const BLAST_API_URL: &'static str =
    "https://starknet-mainnet.blastapi.io/887824dd-2f0b-448d-8549-09598869e9bb";
const INFURA_URL: &'static str =
    "https://starknet-mainnet.infura.io/v3/df11605e57a14558b13a24a111661f52";
const INFURA_TESTNET_URL: &'static str =
    "https://starknet-goerli.infura.io/v3/df11605e57a14558b13a24a111661f52";
const CARMINE_JUNO_NODE_URL: &'static str = "https://34.22.208.73";

pub async fn rpc_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
    node: RpcNode,
) -> Result<Vec<String>, String> {
    let body = build_call_body(contract, entry_point, calldata);
    let url = match node {
        RpcNode::BlastAPI => BLAST_API_URL,
        RpcNode::Infura => INFURA_URL,
        RpcNode::InfuraTestnet => INFURA_TESTNET_URL,
        RpcNode::CarmineJunoNode => CARMINE_JUNO_NODE_URL,
    };
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<RpcResponse<Vec<String>>>()
        .await;

    let rpc_response = match res {
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
    rpc_call(contract, entry_point, calldata, RpcNode::BlastAPI).await
}

pub async fn infura_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, String> {
    rpc_call(contract, entry_point, calldata, RpcNode::Infura).await
}

pub async fn carmine_call(
    contract: Contract,
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, String> {
    rpc_call(contract, entry_point, calldata, RpcNode::CarmineJunoNode).await
}

pub async fn carmine_amm_call(
    entry_point: Entrypoint,
    calldata: Vec<String>,
) -> Result<Vec<String>, String> {
    rpc_call(
        Contract::AMM,
        entry_point,
        calldata,
        RpcNode::CarmineJunoNode,
    )
    .await
}
