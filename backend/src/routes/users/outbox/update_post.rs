use std::time::SystemTime;

use actix_session::Session;
use actix_web::{put, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use tokio::fs::write;

use crate::{
    establish_connection,
    models::Post,
    routes::users::outbox::publish::publish_update,
    user_is_loged_in,
};

#[derive(Deserialize)]
struct UpdatePostRequest {
    body: String,
    title: String,
    visibility: Option<String>,
}

#[put("/posts/{post_id}")]
pub async fn update_post(
    session: Session,
    path: Path<(String, i32)>,
    request: Json<UpdatePostRequest>,
) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let (_, post_id) = path.into_inner();
    let request = request.into_inner();
    let connection = &mut establish_connection();

    let previous = posts
        .find(post_id)
        .select(Post::as_select())
        .first::<Post>(connection)
        .optional();

    let previous = match previous {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    let post_visibility = match request.visibility.as_deref() {
        Some("public") => "public",
        Some("unlisted") => "unlisted",
        Some("followers") => "followers",
        Some("private") => "private",
        Some(_) => return HttpResponse::BadRequest().body("Invalid visibility"),
        None => previous.visibility.as_str(),
    };

    let query_result = diesel::update(posts.find(post_id))
        .set((
            title.eq(&request.title),
            body.eq(&request.body),
            is_public.eq(post_visibility == "public" || post_visibility == "unlisted"),
            visibility.eq(post_visibility),
            updated_at.eq(SystemTime::now()),
        ))
        .returning(Post::as_returning())
        .get_result::<Post>(connection);

    match query_result {
        Ok(post) => {
            publish_update(&post, &post.author);
            let gemini_content = format!(
                "# {}\n\n{}\n\n=> /index.gmi HOME",
                post.title, post.body
            );
            let _ = write(format!("content/{}.gmi", post.slug), gemini_content).await;
            HttpResponse::Ok().json(Json(post))
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
