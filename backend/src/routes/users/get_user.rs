use crate::{establish_connection, models::User, routes::activity_pub::actor::Actor};
use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;

#[get("/{user_name}")]
pub async fn get_user(path: Path<String>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let user_name = path.into_inner();
    let query_result = users
        .select(User::as_select())
        .filter(name.eq(user_name))
        .first(connection)
        .optional();

    match query_result {
        Ok(Some(retrieved_user)) => HttpResponse::Ok().json(Json(Actor::from(&retrieved_user))),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
