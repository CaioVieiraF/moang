use std::env;

use crate::{
    establish_connection,
    models::{Claims, NewPost, Post},
    user_is_loged_in,
};
use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use diesel::prelude::*;
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;

#[derive(Deserialize)]
struct NewPostRequest {
    body: String,
    title: String,
    is_public: bool,
}

#[post("")]
pub async fn create_post(
    request: HttpRequest,
    new_post_request: Json<NewPostRequest>,
) -> HttpResponse {
    use crate::schema::posts;
    use crate::schema::users::dsl::*;

    if !user_is_loged_in(request.headers()) {
        return HttpResponse::Unauthorized().finish();
    }

    let token = request
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    let token_secret = env::var("JWT_HASH").expect("JWT_HASH not set!");
    let user_email = jsonwebtoken::decode::<Claims>(
        token.trim_start_matches("Bearer "),
        &DecodingKey::from_secret(token_secret.as_ref()),
        &Validation::default(),
    )
    .unwrap()
    .claims
    .sub;

    let connection = &mut establish_connection();
    let user_id_query = users
        .filter(email.eq(user_email))
        .select(id)
        .first::<String>(connection);

    if user_id_query.is_err() {
        return HttpResponse::NotFound().body("User not found");
    }

    let new_post_request = new_post_request.into_inner();
    let new_post = NewPost {
        title: new_post_request.title,
        body: new_post_request.body,
        is_public: new_post_request.is_public,
        author: user_id_query.unwrap(),
    };
    let query_result = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .execute(connection);

    if query_result.is_ok() {
        HttpResponse::Created().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
