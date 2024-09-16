use crate::{establish_connection, models::Post};
use actix_web::{put, web::Path, HttpResponse};
use diesel::prelude::*;

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
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
