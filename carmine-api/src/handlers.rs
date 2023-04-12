use crate::types::{
    AllNonExpired, AllTradeHistoryResponse, GenericResponse, QueryOptions, TradeHistoryResponse,
};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use carmine_api_core::{network::Network, types::AppState};
use std::sync::{Arc, Mutex};

const TESTNET: &'static str = "testnet";
const MAINNET: &'static str = "mainnet";

fn format_tx(tx: &String) -> String {
    let tmp: String = tx.to_lowercase().chars().skip(2).collect();
    let without_leading_zeroes = tmp.trim_start_matches('0');
    let res = format!("0x{}", without_leading_zeroes);
    res
}

#[get("liveness")]
async fn liveness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("API is alive")
}

#[get("/{network}/all-non-expired")]
pub async fn all_non_expired_handler(
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

#[get("/{network}/trade-history")]
pub async fn trade_history_handler(
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

#[get("/{network}/all-trade-history")]
pub async fn all_trade_history_handler(
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

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("").service(liveness_probe_handler).service(
        web::scope("api")
            .service(all_non_expired_handler)
            .service(trade_history_handler)
            .service(all_trade_history_handler),
    );

    conf.service(scope);
}
