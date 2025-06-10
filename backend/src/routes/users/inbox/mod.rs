mod post_follow;
use actix_web::{web, Scope};

pub fn inbox_router() -> Scope {
    web::scope("inbox").service(post_follow::post_follow)
}
