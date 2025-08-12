use actix_web::{web::Json, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routes::activity_pub::{ObjType, Url};

use super::verify_signature;

#[derive(Deserialize, Serialize)]
pub struct Follow {
    actor: FollowerActor,

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,
}

#[derive(Deserialize, Serialize)]
struct FollowerActor {
    id: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,
}

pub async fn follow_actor(req: HttpRequest, new_follower: Json<Follow>) -> HttpResponse {
    if !verify_signature(req.headers().clone(), req.uri().path()) {
        return HttpResponse::Unauthorized().finish();
    }

    let _follower = new_follower.into_inner();

    HttpResponse::Ok().finish()
}
