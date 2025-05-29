use std::env;

use crate::{
    establish_connection,
    models::Post,
    routes::activity_pub::collection::{OrderedCollectionPageBuilder, OrderedItem},
};
use actix_web::{get, web::Json, HttpResponse};
use diesel::prelude::*;
use dotenv::dotenv;

#[get("posts")]
pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;
    dotenv().ok();

    let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");
    let connection = &mut establish_connection();
    let query_result = posts
        .filter(is_public.eq(true))
        .select(Post::as_select())
        .load(connection);

    match query_result {
        Ok(retreived_posts) => {
            let ordered_items = retreived_posts
                .iter()
                .map(OrderedItem::from)
                .collect::<Vec<OrderedItem>>();
            let all_posts = OrderedCollectionPageBuilder::new()
                .id(format!("{base_url}/outbox/posts"))
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
