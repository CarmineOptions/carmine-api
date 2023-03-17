use crate::{
    response::{AllNonExpired, GenericResponse},
    AppState, QueryOptions,
};
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

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
    let response_data = &data.lock().unwrap().all_non_expired;
    let mut payload = Vec::new();

    for v in response_data {
        let copy = String::from(v);
        payload.push(copy);
    }

    let json_response = AllNonExpired {
        status: "success".to_string(),
        data: payload,
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api").service(all_non_expired_handler);

    conf.service(scope);
}
