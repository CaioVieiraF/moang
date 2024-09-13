mod create_posts;
mod get_posts;

use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("posts")
        .service(get_posts::get_posts)
        .service(create_posts::create_post)
}
