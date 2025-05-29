mod delete_post;
mod inbox;
mod make_public;
mod outbox;

use actix_web::{web, Scope};

pub fn posts_router() -> Scope {
    web::scope("")
        .service(outbox::outbox_router())
        .service(inbox::inbox_router())
        .service(make_public::make_public)
        .service(delete_post::delete_post)
}
