use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("liveness")]
pub async fn liveness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("API is alive")
}
