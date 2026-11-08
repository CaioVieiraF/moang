use actix_web::{web, Scope};

use super::{followers, following, get_user, inbox, outbox, posts, timeline, update_profile};

pub fn user_router() -> Scope {
    web::scope("{user_name}")
        .service(get_user::get_user)
        .service(update_profile::update_profile)
        .service(followers::get_followers)
        .service(following::get_following)
        .service(following::follow)
        .service(following::unfollow)
        .service(posts::get_object)
        .service(posts::get_replies)
        .service(posts::get_likes)
        .service(posts::get_shares)
        .service(timeline::get_timeline)
        .service(outbox::outbox_router())
        .service(inbox::inbox_router())
}
