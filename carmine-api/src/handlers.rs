use crate::types::{AllNonExpired, AppState, GenericResponse, QueryOptions, TradeHistoryResponse};
use actix_web::{get, web, HttpResponse, Responder};
use carmine_api_db::models::TradeHistory;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn format_tx(tx: &String) -> String {
    let tmp: String = tx.to_lowercase().chars().skip(2).collect();
    let without_leading_zeroes = tmp.trim_start_matches('0');
    let res = format!("0x{}", without_leading_zeroes);
    res
}

#[get("liveness")]
async fn liveness_probe_handler() -> impl Responder {
    const MESSAGE: &str = "API is alive";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
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
    let now = Instant::now();
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

    println!(
        "Executed \"trade-history\" in {}ms",
        now.elapsed().as_millis()
    );

    HttpResponse::Ok().json(TradeHistoryResponse {
        status: "success".to_string(),
        data: address_specific_trade_history,
    })
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("").service(liveness_probe_handler).service(
        web::scope("api")
            .service(all_non_expired_handler)
            .service(trade_history_handler),
    );

    conf.service(scope);
}
