use crate::types::{DataResponse, GenericResponse};
use actix_web::{
    get,
    http::header::AcceptEncoding,
    web::{self},
    HttpResponse, Responder,
};
use carmine_api_core::types::AppState;
use std::sync::{Arc, Mutex};

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
