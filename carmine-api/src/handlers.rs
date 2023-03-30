use crate::types::{
    AllNonExpired, AllTradeHistoryResponse, AppState, GenericResponse, QueryOptions,
    TradeHistoryResponse,
};
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use carmine_api_core::types::TradeHistory;
use std::sync::{Arc, Mutex};

fn format_tx(tx: &String) -> String {
    let tmp: String = tx.to_lowercase().chars().skip(2).collect();
    let without_leading_zeroes = tmp.trim_start_matches('0');
    let res = format!("0x{}", without_leading_zeroes);
    res
}

#[get("readiness")]
async fn readiness_probe_handler(
    _opts: web::Query<QueryOptions>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let locked = &data.lock();
    let ready = match locked {
        Ok(app_data) => &app_data.ready,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    if *ready {
        return HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("API is ready");
    }

    HttpResponse::InternalServerError()
        .content_type(ContentType::plaintext())
        .body("API is not ready")
}

#[get("liveness")]
async fn liveness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("API is alive")
}

#[get("all-non-expired")]
pub async fn all_non_expired_handler(
    _opts: web::Query<QueryOptions>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let locked = &data.lock();
    let response_data = match locked {
        Ok(app_data) => &app_data.all_non_expired,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    HttpResponse::Ok().json(AllNonExpired {
        status: "success".to_string(),
        data: response_data,
    })
}

#[get("trade-history")]
pub async fn trade_history_handler(
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
    let all_trade_history = match locked {
        Ok(app_data) => &app_data.trade_history,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    let mut address_specific_trade_history: Vec<&TradeHistory> = vec![];

    for history in all_trade_history {
        if history.caller == address {
            address_specific_trade_history.push(history);
        }
    }

    HttpResponse::Ok().json(TradeHistoryResponse {
        status: "success".to_string(),
        data: address_specific_trade_history,
    })
}

#[get("all-trade-history")]
pub async fn all_trade_history_handler(
    _opts: web::Query<QueryOptions>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let locked = &data.lock();
    let res = match locked {
        Ok(app_data) => &app_data.trade_history,
        _ => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                status: "server_error".to_string(),
                message: "Failed to read AppState".to_string(),
            });
        }
    };

    let mut data: Vec<&TradeHistory> = vec![];

    for history in res {
        data.push(history);
    }

    let length = data.len();

    HttpResponse::Ok().json(AllTradeHistoryResponse {
        status: "success".to_string(),
        data,
        length,
    })
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(liveness_probe_handler)
        .service(readiness_probe_handler)
        .service(
            web::scope("api")
                .service(all_non_expired_handler)
                .service(trade_history_handler)
                .service(all_trade_history_handler),
        );

    conf.service(scope);
}
