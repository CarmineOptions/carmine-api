use actix_web::{http::header::ContentType, route, HttpResponse, Responder};

#[route("/liveness", method = "GET", method = "HEAD")]
pub async fn liveness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("API is alive")
}
