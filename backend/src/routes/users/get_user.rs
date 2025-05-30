use crate::routes::{activity_pub::actor::Actor, users::has_user};
use actix_web::{get, web::Json, web::Path, HttpResponse};

#[get("")]
pub async fn get_user(path: Path<String>) -> HttpResponse {
    let user_name = path.into_inner();
    let query_result = has_user(user_name);

    match query_result {
        Some(retrieved_user) => HttpResponse::Ok()
            .content_type("application/activity+json")
            .json(Json(Actor::from(&retrieved_user))),
        None => HttpResponse::NotFound().finish(),
    }
    //Err(_) => HttpResponse::InternalServerError().finish(),
}
