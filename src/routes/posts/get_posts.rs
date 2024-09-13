use crate::{establish_connection, models::Post};
use actix_web::{get, web::Json, HttpResponse};
use diesel::prelude::*;

#[get("")]
pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let query_result = posts
        .filter(is_public.eq(true))
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts.");

    HttpResponse::Ok().json(Json(query_result))
}
