use std::time::SystemTime;

use crate::{
    establish_connection,
    models::Post,
    routes::users::outbox::publish::{publish_post, publish_update},
    user_is_loged_in,
};
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
    let previous = match posts
        .find(post_id)
        .select(Post::as_select())
        .first::<Post>(connection)
        .optional()
    {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    let query_result = diesel::update(posts.find(post_id))
        .set((
            is_public.eq(true),
            visibility.eq("public"),
            updated_at.eq(SystemTime::now()),
        ))
        .returning(Post::as_returning())
        .get_result::<Post>(connection);

    match query_result {
        Ok(post) => {
            if previous.visibility == "private" {
                publish_post(&post, &post.author);
            } else {
                publish_update(&post, &post.author);
            }
            HttpResponse::Accepted().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
