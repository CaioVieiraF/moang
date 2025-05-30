use crate::{
    establish_connection,
    models::Post,
    routes::{
        activity_pub::{
            collection::{OrderedCollectionPageBuilder, OrderedItem, OrderedItemBuilder},
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
use dotenv::dotenv;

#[get("/posts")]
pub async fn get_posts(path: Path<String>, req: HttpRequest) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    dotenv().ok();

    if has_user(path.into_inner()).is_none() {
        return HttpResponse::NotFound().finish();
    }

    let connection = &mut establish_connection();
    let query_result = posts
        .filter(is_public.eq(true))
        .select(Post::as_select())
        .load(connection);

    match query_result {
        Ok(retreived_posts) => {
            let ordered_items = retreived_posts
                .iter()
                .map(|post| {
                    let date: DateTime<Utc> = post.created_at.into();
                    let post_id = format!("{}/{}", req.full_url(), post.id);
                    let post = OrderedItemBuilder::new()
                        .id(post_id)
                        .published(format!("{}", date.format("%+")))
                        .object(ContentObject::from(post))
                        .name(post.title.clone())
                        .build();
                    post
                })
                .collect::<Vec<OrderedItem>>();

            let all_posts = OrderedCollectionPageBuilder::new()
                .id(req.full_url())
                .total_items(ordered_items.len() as i64)
                .ordered_items(ordered_items)
                .build();

            HttpResponse::Ok()
                .content_type(
                    "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                )
                .json(Json(all_posts))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Server error: {e}")),
    }
}
