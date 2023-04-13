use crate::{
    handlers::format_tx,
    types::{
        AllNonExpired, AllTradeHistoryResponse, GenericResponse, QueryOptions, TradeHistoryResponse,
    },
};
use actix_web::{get, web, HttpResponse, Responder};
use carmine_api_core::types::{AppState, TradeHistory};
use std::sync::{Arc, Mutex};

#[get("all-non-expired")]
pub async fn all_non_expired_handler(
    _opts: web::Query<QueryOptions>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let locked = &data.lock();
    let response_data = match locked {
        Ok(app_data) => &app_data.testnet.all_non_expired,
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
        Ok(app_data) => &app_data.testnet.trade_history,
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
        Ok(app_data) => &app_data.testnet.trade_history,
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
