use crate::{
    config::AppConfig,
    current_user_id, establish_connection,
    models::Post,
    routes::users::outbox::publish::publish_delete,
    schema::{remote_interactions, remote_posts, users},
    user_is_loged_in,
};
use actix_session::Session;
use actix_web::{delete, web::Path, HttpResponse};
use diesel::prelude::*;
use tokio::{
    fs::{read_to_string, remove_file, File},
    io::AsyncWriteExt,
};

#[delete("/{post_id}")]
pub async fn delete_post(session: Session, path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let Some(user_id) = current_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let connection = &mut establish_connection();
    let post_id = path.into_inner();

    let post = match posts
        .find(post_id)
        .select(Post::as_select())
        .first::<Post>(connection)
        .optional()
    {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().body("Post not found"),
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    if post.author != user_id {
        return HttpResponse::Forbidden().finish();
    }

    publish_delete(&post, &post.author);

    if let Err(error) = cleanup_remote_data(connection, &post) {
        return HttpResponse::InternalServerError().body(error.to_string());
    }

    if let Err(error) = delete_gemini_post(post.slug.clone()).await {
        log::warn!("could not remove gemini mirror for {}: {error}", post.slug);
    }

    match diesel::delete(posts.find(post_id)).execute(connection) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Replies and reactions received for a local object are useless once the
/// object is gone, so they are removed together with the post.
fn cleanup_remote_data(
    connection: &mut diesel::PgConnection,
    post: &Post,
) -> Result<(), diesel::result::Error> {
    let username = users::table
        .find(&post.author)
        .select(users::name)
        .first::<String>(connection)?;
    let object_url = AppConfig::get().object_url(&username, &post.ap_id);

    diesel::delete(remote_posts::table.filter(remote_posts::in_reply_to.eq(&object_url)))
        .execute(connection)?;
    diesel::delete(
        remote_interactions::table.filter(remote_interactions::object_id.eq(&object_url)),
    )
    .execute(connection)?;

    Ok(())
}

async fn delete_gemini_post(post_slug: String) -> tokio::io::Result<()> {
    remove_file(format!("content/{post_slug}.gmi")).await?;

    let gemini_index = match read_to_string("content/index.gmi").await {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };

    let mut new_gemini_index_content = String::new();
    for line in gemini_index.lines() {
        if !line.contains(post_slug.as_str()) {
            new_gemini_index_content.push_str(format!("{line}\n").as_str());
        }
    }

    let mut new_gemini_index = File::create("content/index.gmi").await?;
    new_gemini_index
        .write_all(new_gemini_index_content.as_ref())
        .await?;

    Ok(())
}
