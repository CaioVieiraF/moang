use crate::{
    routes::{
        activity_pub::{actor::Actor, keys::get_or_create_key},
        users::has_user,
    },
};
use actix_web::{get, web::Json, web::Path, HttpResponse};

#[get("")]
pub async fn get_user(path: Path<String>) -> HttpResponse {
    let user_name = path.into_inner().to_lowercase();
    let Some(retrieved_user) = has_user(user_name) else {
        return HttpResponse::NotFound().finish();
    };

    let key = match get_or_create_key(&retrieved_user.id) {
        Ok(key) => key,
        Err(error) => return HttpResponse::InternalServerError().body(error),
    };

    match Actor::from_user(&retrieved_user, &key) {
        Ok(actor) => HttpResponse::Ok()
            .content_type("application/activity+json")
            .json(Json(actor)),
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}
