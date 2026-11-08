use crate::{
    current_user_id, establish_connection,
    models::{NewPost, Post},
    user_is_loged_in,
};
use actix_session::Session;
use actix_web::{post, web::Json, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use slug::slugify;
use tokio::fs::{read_to_string, write};

#[derive(Deserialize)]
struct NewPostRequest {
    body: String,
    title: String,
    is_public: bool,
    visibility: Option<String>,
    kind: Option<String>,
}

#[post("")]
pub async fn create_post(session: Session, new_post_request: Json<NewPostRequest>) -> HttpResponse {
    use crate::schema::posts;

    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let Some(author) = current_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let new_post_request = new_post_request.into_inner();
    let visibility = match new_post_request.visibility.as_deref() {
        Some("public") => "public",
        Some("unlisted") => "unlisted",
        Some("followers") => "followers",
        Some("private") => "private",
        Some(_) => return HttpResponse::BadRequest().body("Invalid visibility"),
        None if new_post_request.is_public => "public",
        None => "private",
    };

    let kind = match new_post_request.kind.as_deref() {
        Some("article") => "article",
        Some("microblog") => "microblog",
        Some(_) => return HttpResponse::BadRequest().body("Invalid kind"),
        None => "article",
    };

    let new_post = NewPost {
        title: new_post_request.title.clone(),
        body: new_post_request.body,
        is_public: visibility == "public" || visibility == "unlisted",
        slug: slugify(new_post_request.title),
        author,
        visibility: visibility.to_string(),
        kind: kind.to_string(),
    };

    let connection = &mut establish_connection();
    let post = match diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result::<Post>(connection)
    {
        Ok(post) => post,
        Err(error) => {
            log::error!("error creating post: {error}");
            return HttpResponse::InternalServerError().body("Error creating post on DB");
        }
    };

    // The Gemini mirror is optional. A missing `content/index.gmi` or a
    // filesystem problem must never prevent the post from being federated.
    if let Err(error) = create_gemini_post(&new_post).await {
        log::warn!("could not write gemini mirror for {}: {error}", post.slug);
    }

    crate::routes::users::outbox::publish::publish_post(&post, &post.author);

    HttpResponse::Created().finish()
}

async fn create_gemini_post(post: &NewPost) -> tokio::io::Result<()> {
    let mut gemini_content = format!("# {}\n\n", &post.title);
    gemini_content.push_str(&post.body);
    gemini_content.push_str("\n\n=> /index.gmi HOME");
    write(format!("content/{}.gmi", &post.slug), gemini_content).await?;

    let mut gemini_index_content = match read_to_string("content/index.gmi").await {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => "# Moang\n".to_string(),
        Err(error) => return Err(error),
    };
    gemini_index_content.push_str(&format!("\n=> /{}.gmi {}", &post.slug, &post.title));
    write("content/index.gmi", gemini_index_content).await?;

    Ok(())
}
