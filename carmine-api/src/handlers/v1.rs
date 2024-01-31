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
use carmine_api_core::{
    network::Network,
    types::{AppState, NewReferralEvent},
};
use carmine_api_db::{create_referral_event, get_referral_code};
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

#[get("/{network}/live-options")]
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

#[get("/{network}/transactions")]
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

#[get("/{network}/all-transactions")]
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

#[get("/mainnet/airdrop")]
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

#[get("/mainnet/referral_events")]
pub async fn get_referral_events(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
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
        data: &app_state.mainnet.referrals,
    })
}

#[get("/mainnet/{pool}")]
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

#[get("/mainnet/{pool}/state")]
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

#[get("/mainnet/{pool}/apy")]
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
                    data: apy.week_annualized,
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

#[get("/mainnet/option-volatility")]
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

#[get("/mainnet/prices/{pair_id}")]
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

#[post("/{network}/call")]
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

#[get("/mainnet/get_referral")]
pub async fn get_referral(opts: web::Query<QueryOptions>) -> impl Responder {
    let address = match &opts.address {
        Some(address) => format_tx(address),
        _ => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "Did not receive address as a query parameter".to_string(),
            });
        }
    };

    let referral_code = get_referral_code(address);

    HttpResponse::Ok().json(DataResponse::<String> {
        status: "success".to_string(),
        data: referral_code,
    })
}

#[post("/mainnet/referral_event")]
async fn referral_event(payload: Option<web::Bytes>) -> impl Responder {
    let bytes = match payload {
        Some(v) => v,
        None => {
            return HttpResponse::BadRequest().json(GenericResponse {
                status: "bad_request".to_string(),
                message: "No payload was provided".to_string(),
            })
        }
    };

    match serde_json::from_slice::<NewReferralEvent>(&bytes) {
        Ok(mut event) => {
            let unsafe_address = event.referred_wallet_address;
            let safe_address = format_tx(&unsafe_address.to_owned());
            event.referred_wallet_address = &safe_address;

            match create_referral_event(event) {
                Ok(_) => HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Event stored".to_string(),
                }),
                Err(_) => HttpResponse::BadRequest().json(GenericResponse {
                    status: "bad_request".to_string(),
                    message: "Referal does not exist".to_string(),
                }),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(GenericResponse {
            status: "bad_request".to_string(),
            message: "Could not parse payload".to_string(),
        }),
    }
}
