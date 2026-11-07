use actix_web::{web, Scope};

use super::{followers, following, get_user, inbox, outbox, posts, timeline};

pub fn user_router() -> Scope {
    web::scope("{user_name}")
        .service(get_user::get_user)
        .service(followers::get_followers)
        .service(following::get_following)
        .service(following::follow)
        .service(following::unfollow)
        .service(posts::get_object)
        .service(posts::get_replies)
        .service(timeline::get_timeline)
        .service(outbox::outbox_router())
        .service(inbox::inbox_router())
}
