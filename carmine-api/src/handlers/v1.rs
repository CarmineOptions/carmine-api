use crate::{
    handlers::format_tx,
    types::{
        AllNonExpired, AllTradeHistoryResponse, DataResponse, GenericResponse, QueryOptions,
        TradeHistoryResponse,
    },
};
use actix_web::{
    get,
    http::header::AcceptEncoding,
    post,
    web::{self},
    HttpResponse, Responder,
};
use carmine_api_core::{network::Network, types::AppState};
use lazy_static::lazy_static;
use std::{
    env,
    sync::{Arc, Mutex},
};

lazy_static! {
    static ref BLAST_API_URL: String =
        env::var("BLAST_API_URL").expect("missing env var BLAST_API_URL");
    static ref INFURA_URL: String = env::var("INFURA_URL").expect("missing env var INFURA_URL");
    static ref INFURA_TESTNET_URL: String =
        env::var("INFURA_TESTNET_URL").expect("missing env var INFURA_TESTNET_URL");
    static ref CARMINE_JUNO_NODE_URL: String =
        env::var("CARMINE_JUNO_NODE_URL").expect("missing env var CARMINE_JUNO_NODE_URL");
    static ref CARMINE_JUNO_TESTNET_NODE_URL: String = env::var("CARMINE_JUNO_TESTNET_NODE_URL")
        .expect("missing env var CARMINE_JUNO_TESTNET_NODE_URL");
}

const TESTNET: &'static str = "testnet";
const MAINNET: &'static str = "mainnet";

#[get("/v1/{network}/live-options")]
pub async fn live_options(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let network = match path.into_inner().as_str() {
        TESTNET => Network::Testnet,
        MAINNET => Network::Mainnet,
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Specify network in the path".to_string(),
            });
        }
    };
    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };
    let data = match network {
        Network::Testnet => &app_state.testnet.all_non_expired,
        Network::Mainnet => &app_state.mainnet.all_non_expired,
    };

    HttpResponse::Ok().json(AllNonExpired {
        status: "success".to_string(),
        data,
    })
}

#[get("/v1/{network}/transactions")]
pub async fn transactions(
    opts: web::Query<QueryOptions>,
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let network = match path.into_inner().as_str() {
        TESTNET => Network::Testnet,
        MAINNET => Network::Mainnet,
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Specify network in the path".to_string(),
            });
        }
    };
    let address = match &opts.address {
        Some(address) => format_tx(address),
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Did not receive address as a query parameter".to_string(),
            });
        }
    };
    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };
    let all_history = match network {
        Network::Testnet => &app_state.testnet.trade_history,
        Network::Mainnet => &app_state.mainnet.trade_history,
    };

    let data = all_history.iter().filter(|h| h.caller == address).collect();

    HttpResponse::Ok().json(TradeHistoryResponse {
        status: "success".to_string(),
        data,
    })
}

#[get("/v1/{network}/all-transactions")]
pub async fn all_transactions(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let network = match path.into_inner().as_str() {
        TESTNET => Network::Testnet,
        MAINNET => Network::Mainnet,
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Specify network in the path".to_string(),
            });
        }
    };
    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    let data = match network {
        Network::Testnet => &app_state.testnet.trade_history,
        Network::Mainnet => &app_state.mainnet.trade_history,
    };

    let length = data.len();

    HttpResponse::Ok().json(AllTradeHistoryResponse {
        status: "success".to_string(),
        data: data.iter().collect(),
        length,
    })
}

#[get("/v1/mainnet/airdrop")]
pub async fn airdrop(
    opts: web::Query<QueryOptions>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let address = match &opts.address {
        Some(address) => format_tx(address),
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Did not receive address as a query parameter".to_string(),
            });
        }
    };
    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    let data = match app_state.airdrop.address_calldata(&address) {
        Ok(v) => v,
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Address not on the list".to_string(),
            });
        }
    };

    HttpResponse::Ok().json(DataResponse {
        status: "success".to_string(),
        data,
    })
}

#[get("/v1/mainnet/{pool}")]
pub async fn pool_state(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let pool_id = path.into_inner();

    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    match app_state.mainnet.state.get(&pool_id) {
        Some(state) => {
            // found state
            return HttpResponse::Ok()
                .insert_header(AcceptEncoding(vec!["gzip".parse().unwrap()]))
                .json(DataResponse {
                    status: "success".to_string(),
                    data: state,
                });
        }
        None => {
            // invalid pool
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Invalid pool".to_string(),
            });
        }
    }
}

#[get("/v1/mainnet/{pool}/state")]
pub async fn pool_state_last(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let pool_id = path.into_inner();

    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    let state = match app_state.mainnet.state.get(&pool_id) {
        Some(state) => state,
        None => {
            // invalid pool
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Invalid pool".to_string(),
            });
        }
    };

    let max_element = state.iter().max_by_key(|v| v.block_number);

    match max_element {
        Some(latest) => HttpResponse::Ok().json(DataResponse {
            status: "success".to_string(),
            data: latest,
        }),
        None => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "No data".to_string(),
            });
        }
    }
}

#[get("/v1/mainnet/{pool}/apy")]
pub async fn pool_apy(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let pool_id = path.into_inner();

    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    match app_state.mainnet.apy.get(&pool_id) {
        Some(apy) => {
            // found state
            return HttpResponse::Ok()
                .insert_header(AcceptEncoding(vec!["gzip".parse().unwrap()]))
                .json(DataResponse {
                    status: "success".to_string(),
                    data: apy,
                });
        }
        None => {
            // invalid pool
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Invalid pool".to_string(),
            });
        }
    }
}

#[get("/v1/mainnet/option-volatility")]
pub async fn option_volatility(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    HttpResponse::Ok().json(DataResponse {
        status: "success".to_string(),
        data: &app_state.mainnet.option_volatility,
    })
}

#[get("/v1/mainnet/prices/{pair_id}")]
pub async fn prices(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let pair_id = path.into_inner();
    let locked = &data.lock();
    let app_state = match locked {
        Ok(app_data) => app_data,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    HttpResponse::Ok().json(DataResponse {
        status: "success".to_string(),
        data: &app_state.mainnet.oracle_prices.get(&pair_id),
    })
}

#[post("/v1/{network}/call")]
async fn proxy_call(path: web::Path<String>, payload: Option<web::Bytes>) -> impl Responder {
    let network = match path.into_inner().as_str() {
        TESTNET => Network::Testnet,
        MAINNET => Network::Mainnet,
        unknown_network => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: format!("Unknown network {}", unknown_network),
            });
        }
    };

    let client = reqwest::Client::new();

    let some_payload = match payload {
        Some(data) => data,
        None => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "No payload was provided".to_string(),
            })
        }
    };

    let carmine_juno_url = match network {
        Network::Mainnet => CARMINE_JUNO_NODE_URL.as_str(),
        Network::Testnet => CARMINE_JUNO_TESTNET_NODE_URL.as_str(),
    };

    // proxy to Carmine Juno Node
    let juno_res = client
        .post(carmine_juno_url)
        .body(some_payload.to_vec())
        .send()
        .await;

    if let Ok(ok_response) = juno_res {
        let parsed_response = ok_response.bytes().await;
        if let Ok(bytes) = parsed_response {
            return HttpResponse::Ok().body(bytes);
        }
    }

    let infura_node_url = match network {
        Network::Mainnet => INFURA_URL.as_str(),
        Network::Testnet => INFURA_TESTNET_URL.as_str(),
    };

    // if Carmine Juno Node did not succeed proxy to Infura Node
    let infura_res = client
        .post(infura_node_url)
        .body(some_payload.to_vec())
        .send()
        .await;

    if let Ok(ok_response) = infura_res {
        let parsed_response = ok_response.bytes().await;
        if let Ok(bytes) = parsed_response {
            return HttpResponse::Ok().body(bytes);
        }
    }

    // if neither succeeded return internal server error
    HttpResponse::InternalServerError().json(GenericResponse {
        status: "error".to_string(),
        message: "Failed to get response from RPC Nodes".to_string(),
    })
}
