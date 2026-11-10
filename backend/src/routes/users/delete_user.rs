use crate::{
    current_user_id, establish_connection,
    models::Post,
    routes::posts::delete_post::{cleanup_remote_data, delete_gemini_post},
    schema::{posts, users},
    user_is_loged_in,
};
use actix_session::Session;
use actix_web::{delete, web::Path, HttpResponse};
use diesel::prelude::*;

#[delete("/{user_id}")]
pub async fn delete_user(session: Session, path: Path<String>) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let user_id = path.into_inner();
    if current_user_id(&session).as_deref() != Some(user_id.as_str()) {
        return HttpResponse::Forbidden().finish();
    }

    let connection = &mut establish_connection();
    let user_posts = match posts::table
        .filter(posts::author.eq(&user_id))
        .select(Post::as_select())
        .load::<Post>(connection)
    {
        Ok(user_posts) => user_posts,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    let deleted = connection.transaction::<_, diesel::result::Error, _>(|connection| {
        for post in &user_posts {
            cleanup_remote_data(connection, post)?;
        }
        diesel::delete(posts::table.filter(posts::author.eq(&user_id))).execute(connection)?;
        diesel::delete(users::table.find(&user_id)).execute(connection)?;
        Ok(())
    });

    if let Err(error) = deleted {
        return HttpResponse::InternalServerError().body(error.to_string());
    }

    for post in &user_posts {
        if let Err(error) = delete_gemini_post(post.slug.clone()).await {
            log::warn!("could not remove gemini mirror for {}: {error}", post.slug);
        }
    }

    HttpResponse::Ok().finish()
}
