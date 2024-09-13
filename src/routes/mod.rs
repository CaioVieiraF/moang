mod posts;
use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("posts").service(posts::posts_router())
}
