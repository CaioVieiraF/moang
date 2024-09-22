use crate::{establish_connection, user_is_loged_in};
use actix_web::{delete, web::Path, HttpRequest, HttpResponse};
use diesel::prelude::*;

#[delete("/{post_id}")]
pub async fn delete_post(request: HttpRequest, path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    if !user_is_loged_in(request.headers()) {
        return HttpResponse::Unauthorized().finish();
    }

    let connection = &mut establish_connection();
    let post_id = path.into_inner();
    let query_result = diesel::delete(posts.find(post_id)).execute(connection);

    match query_result {
        Ok(_) => {
            let _ = delete_gemini_post(post_id).await;
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_gemini_post(post_id: i32) -> tokio::io::Result<()> {
    use tokio::fs::remove_file;
    remove_file(format!("{post_id}.gmi")).await
}
