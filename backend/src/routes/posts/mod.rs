mod create_post;
mod delete_post;
mod get_outbox;
mod get_post;
mod get_posts;
mod make_public;

use actix_web::{web, Scope};

pub fn outbox_router() -> Scope {
    web::scope("outbox")
        .service(get_outbox::get_outbox)
        .service(get_posts::get_posts)
        .service(create_post::create_post)
        .service(get_post::get_post)
        .service(make_public::make_public)
        .service(delete_post::delete_post)
}
