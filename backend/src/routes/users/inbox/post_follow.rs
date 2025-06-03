use actix_web::{post, web::Json, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::ActivityPub;

#[derive(Deserialize, Serialize)]
struct Follow {
    #[serde(flatten)]
    activity_pub: ActivityPub,
}

#[post("")]
pub async fn post_follow(new_follower: Json<Follow>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
