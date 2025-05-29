use std::env;

use actix_web::{get, HttpResponse};
use diesel::{QueryDsl, RunQueryDsl};
use dotenv::dotenv;

use crate::{establish_connection, routes::activity_pub::collection::OrderedCollectionBuilder};

#[get("")]
pub async fn get_outbox() -> HttpResponse {
    use crate::schema::posts::dsl::*;
    dotenv().ok();

    let connection = &mut establish_connection();
    let query_result = posts.count().get_result(connection);

    let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");
    let outbox_id = format!("{base_url}/outbox");
    let first = format!("{base_url}/outbox/posts");

    match query_result {
        Ok(total_items) => {
            let outbox = OrderedCollectionBuilder::new()
                .id(outbox_id)
                .first(first)
                .total_items(total_items)
                .build();

            HttpResponse::Ok()
                .content_type(
                    "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                )
                .json(outbox)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
