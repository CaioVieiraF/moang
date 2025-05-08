mod create_post;
mod delete_post;
mod get_post;
mod get_posts;
mod make_public;

use actix_web::{web, Scope};

pub fn posts_router() -> Scope {
    web::scope("posts")
        .service(get_posts::get_posts)
        .service(create_post::create_post)
        .service(get_post::get_post)
        .service(make_public::make_public)
        .service(delete_post::delete_post)
}
