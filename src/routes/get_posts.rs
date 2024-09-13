use actix_web::{get, web::Json, HttpResponse};
use serde::Serialize;

use crate::{establish_connection, models::Post};
use diesel::prelude::*;

#[derive(Serialize)]
struct Posts {
    posts: Vec<Post>,
}

#[get("")]
pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let query_result = posts
        .filter(is_public.eq(true))
        .select(Post::as_select())
        .load(connection)
        .expect("");

    let all_posts = Posts {
        posts: query_result,
    };

    HttpResponse::Ok().json(Json(all_posts))
}
