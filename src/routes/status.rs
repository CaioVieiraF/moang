use actix_web::{get, web::Json, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct StatusResponse {
    status_code: u16,
}

#[get("/status")]
async fn get_status() -> impl Responder {
    let status = StatusResponse { status_code: 200 };
    HttpResponse::Ok().json(Json(status))
}
