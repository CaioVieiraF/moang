use actix_web::{get, web::Json, HttpResponse};
use diesel::prelude::*;
use serde_json::json;

use crate::{config::AppConfig, establish_connection};

#[get("/.well-known/nodeinfo")]
pub async fn nodeinfo_discovery() -> HttpResponse {
    let config = AppConfig::get();

    HttpResponse::Ok().json(json!({
        "links": [
            {
                "rel": "http://nodeinfo.diaspora.software/ns/schema/2.1",
                "href": config.nodeinfo_url()
            }
        ]
    }))
}

#[get("/nodeinfo/2.1")]
pub async fn nodeinfo() -> HttpResponse {
    use crate::schema::{posts, users};

    let connection = &mut establish_connection();
    let user_count = users::table
        .count()
        .get_result::<i64>(connection)
        .unwrap_or(0);
    let post_count = posts::table
        .filter(posts::is_public.eq(true))
        .count()
        .get_result::<i64>(connection)
        .unwrap_or(0);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(Json(json!({
            "version": "2.1",
            "software": {
                "name": "moang-blog",
                "version": env!("CARGO_PKG_VERSION")
            },
            "protocols": ["activitypub"],
            "services": {
                "inbound": [],
                "outbound": []
            },
            "openRegistrations": false,
            "usage": {
                "users": {
                    "total": user_count
                },
                "localPosts": post_count
            },
            "metadata": {}
        })))
}
