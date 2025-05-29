use std::env;

use actix_web::{get, HttpResponse};
use dotenv::dotenv;

use crate::routes::activity_pub::collection::OrderedCollectionBuilder;

#[get("")]
pub async fn get_outbox() -> HttpResponse {
    dotenv().ok();

    let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");
    let id = format!("{base_url}/outbox");
    let first = format!("{base_url}/outbox/posts");

    let outbox = OrderedCollectionBuilder::new()
        .id(id)
        .first(first)
        .total_items(4)
        .build();

    HttpResponse::Ok().json(outbox)
}
