mod delete_post;
mod make_public;

use actix_web::{web, Scope};

pub fn posts_router() -> Scope {
    web::scope("")
        .service(make_public::make_public)
        .service(delete_post::delete_post)
}
