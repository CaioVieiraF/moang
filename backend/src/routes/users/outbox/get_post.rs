use crate::{
    establish_connection,
    models::Post,
    routes::{
        activity_pub::collection::OrderedItem,
        users::has_user,
    },
};
use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;

#[get("/posts/{post_id}")]
pub async fn get_post(path: Path<(String, i32)>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let (username, post_id) = path.into_inner();
    let username = username.to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let connection = &mut establish_connection();
    let query_result = posts
        .find(post_id)
        .filter(author.eq(&user.id))
        .select(Post::as_select())
        .first(connection)
        .optional();

    match query_result {
        Ok(Some(retrieved_post)) => match OrderedItem::from_post(&retrieved_post, &username) {
            Ok(activity) => HttpResponse::Ok()
                .content_type(
                    "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                )
                .json(Json(activity)),
            Err(error) => HttpResponse::InternalServerError().body(error),
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
