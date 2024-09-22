use crate::{
    establish_connection,
    models::{Claims, NewPost, Post},
    user_is_loged_in,
};
use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use diesel::prelude::*;
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use slug::slugify;
use std::env;
use tokio::fs::{read_to_string, write};

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
        title: new_post_request.title.clone(),
        body: new_post_request.body,
        is_public: new_post_request.is_public,
        slug: slugify(new_post_request.title),
        author: user_id_query.unwrap(),
    };
    let query_result = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .execute(connection);

    if query_result.is_ok() {
        let gemini_post = create_gemini_post(new_post).await;
        if gemini_post.is_err() {
            HttpResponse::InternalServerError().body("Error creating gemini post")
        } else {
            HttpResponse::Created().finish()
        }
    } else {
        HttpResponse::InternalServerError().body("Error creating post on DB")
    }
}

async fn create_gemini_post(post: NewPost) -> tokio::io::Result<()> {
    let mut gemini_content = format!("# {}\n\n", &post.title);
    gemini_content.push_str(&post.body);
    gemini_content.push_str("\n\n=> /index.gmi HOME");
    write(format!("content/{}.gmi", &post.slug), gemini_content).await?;

    let mut gemini_index_content = read_to_string("content/index.gmi").await?;
    gemini_index_content.push_str(format!("\n=> /{}.gmi {}", &post.slug, &post.title).as_str());
    write("content/index.gmi", gemini_index_content).await?;

    Ok(())
}
