mod create_post;
mod get_outbox;
mod get_post;
mod get_posts;

use actix_web::{web, Scope};

pub fn outbox_router() -> Scope {
    web::scope("outbox")
        .service(get_outbox::get_outbox)
        .service(get_post::get_post)
        .service(create_post::create_post)
        .service(get_posts::get_posts)
}
