use crate::{
    handlers::format_tx,
    types::{
        AllNonExpired, AllTradeHistoryResponse, GenericResponse, QueryOptions, TradeHistoryResponse,
    },
};
use actix_web::{get, web, HttpResponse, Responder};
use carmine_api_core::{network::Network, types::AppState};
use std::sync::{Arc, Mutex};

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
