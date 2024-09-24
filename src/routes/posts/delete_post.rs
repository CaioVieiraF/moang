use crate::{establish_connection, models::Post, user_is_loged_in};
use actix_web::{delete, web::Path, HttpRequest, HttpResponse};
use diesel::prelude::*;
use tokio::{
    fs::{read_to_string, remove_file, File},
    io::AsyncWriteExt,
};

#[delete("/{post_id}")]
pub async fn delete_post(request: HttpRequest, path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    if !user_is_loged_in(request.headers()) {
        return HttpResponse::Unauthorized().finish();
    }

    let connection = &mut establish_connection();
    let post_id = path.into_inner();

    let post_query_result = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    if let Ok(retrieved_post) = post_query_result {
        if let Some(post_that_exists) = retrieved_post {
            let post_slug = post_that_exists.slug;
            let _ = delete_gemini_post(post_slug).await;
        } else {
            return HttpResponse::NotFound().body("Post not found");
        }
    }

    let post_delete_result = diesel::delete(posts.find(post_id)).execute(connection);

    match post_delete_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_gemini_post(post_slug: String) -> tokio::io::Result<()> {
    remove_file(format!("content/{post_slug}.gmi")).await?;

    let gemini_index = read_to_string("content/index.gmi").await?;
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
