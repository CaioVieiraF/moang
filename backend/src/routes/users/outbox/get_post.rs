use crate::{
    establish_connection,
    models::Post,
    routes::{
        activity_pub::{
            collection::{OrderedItem, OrderedItemBuilder},
            content_object::ContentObject,
        },
        users::has_user,
    },
};
use actix_web::{
    get,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[get("/{post_id}")]
pub async fn get_post(path: Path<(String, i32)>, req: HttpRequest) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let (username, post_id) = path.into_inner();
    if has_user(username).is_none() {
        return HttpResponse::NotFound().finish();
    }

    let connection = &mut establish_connection();
    let query_result = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match query_result {
        Ok(Some(retrieved_post)) => {
            let date: DateTime<Utc> = retrieved_post.created_at.into();
            let activity = OrderedItemBuilder::new()
                .id(req.full_url())
                .published(format!("{}", date.format("%+")))
                .object(ContentObject::from(&retrieved_post))
                .name(retrieved_post.title.clone())
                .build();

            HttpResponse::Ok()
                .content_type(
                    "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                )
                .json(Json(activity))
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
