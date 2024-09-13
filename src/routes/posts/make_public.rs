use crate::{establish_connection, models::Post};
use actix_web::{put, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct UpdatedPost {
    id: i32,
    message: String,
}

#[put("/{post_id}/make_public")]
pub async fn make_public(path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let post_id = path.into_inner();
    let query_result = diesel::update(posts.find(post_id))
        .set(is_public.eq(true))
        .returning(Post::as_returning())
        .get_result(connection);

    match query_result {
        Ok(updated_post) => {
            let update_response = UpdatedPost {
                id: updated_post.id,
                message: "Post is now public".to_string(),
            };

            HttpResponse::Ok().json(Json(update_response))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
