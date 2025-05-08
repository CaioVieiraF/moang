use crate::{establish_connection, models::Post};
use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;

#[get("/{post_id}")]
pub async fn get_post(path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let post_id = path.into_inner();
    let query_result = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match query_result {
        Ok(Some(retrieved_post)) => HttpResponse::Ok().json(Json(retrieved_post)),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
