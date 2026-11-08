use std::time::SystemTime;

use actix_session::Session;
use actix_web::{put, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use tokio::fs::write;

use crate::{
    current_user_id, establish_connection,
    models::Post,
    routes::users::outbox::publish::publish_update,
    user_is_loged_in,
};

/// Partial update: omitted fields keep their current value.
#[derive(Deserialize)]
struct UpdatePostRequest {
    body: Option<String>,
    title: Option<String>,
    visibility: Option<String>,
    kind: Option<String>,
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

    let Some(user_id) = current_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };
    if previous.author != user_id {
        return HttpResponse::Forbidden().finish();
    }

    let post_visibility = match request.visibility.as_deref() {
        Some("public") => "public",
        Some("unlisted") => "unlisted",
        Some("followers") => "followers",
        Some("private") => "private",
        Some(_) => return HttpResponse::BadRequest().body("Invalid visibility"),
        None => previous.visibility.as_str(),
    };

    let post_kind = match request.kind.as_deref() {
        Some("article") => "article",
        Some("microblog") => "microblog",
        Some(_) => return HttpResponse::BadRequest().body("Invalid kind"),
        None => previous.kind.as_str(),
    };

    let post_title = request.title.unwrap_or_else(|| previous.title.clone());
    let post_body = request.body.unwrap_or_else(|| previous.body.clone());

    let query_result = diesel::update(posts.find(post_id))
        .set((
            title.eq(&post_title),
            body.eq(&post_body),
            is_public.eq(post_visibility == "public" || post_visibility == "unlisted"),
            visibility.eq(post_visibility),
            kind.eq(post_kind),
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
