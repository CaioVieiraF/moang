use crate::establish_connection;
use actix_web::{delete, web::Path, HttpResponse};
use diesel::prelude::*;

#[delete("/{post_id}")]
pub async fn delete_post(path: Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let post_id = path.into_inner();
    let query_result = diesel::delete(posts.find(post_id)).execute(connection);

    match query_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
