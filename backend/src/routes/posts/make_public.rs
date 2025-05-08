use crate::{establish_connection, models::Post, user_is_loged_in};
use actix_session::Session;
use actix_web::{put, web::Path, HttpResponse};
use diesel::prelude::*;

#[put("/{post_id}/make_public")]
pub async fn make_public(session: Session, path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

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
