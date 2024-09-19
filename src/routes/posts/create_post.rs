use actix_web::{post, web::Json, HttpResponse};

use crate::{
    establish_connection,
    models::{NewPost, Post},
};
use diesel::prelude::*;

#[post("")]
pub async fn create_post(new_post: Json<NewPost>) -> HttpResponse {
    use crate::schema::posts;

    let new_post = new_post.into_inner();
    let connection = &mut establish_connection();
    let query_result = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .execute(connection);

    match query_result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
