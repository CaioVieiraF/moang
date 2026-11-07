pub mod activity_pub;
mod login;
mod nodeinfo;
mod posts;
mod users;
mod webfinger;
use actix_web::{
    web::{self},
    Scope,
};

pub fn router() -> Scope {
    web::scope("")
        .service(webfinger::webfinger)
        .service(nodeinfo::nodeinfo_discovery)
        .service(nodeinfo::nodeinfo)
        .service(login::login)
        .service(activity_pub::inbox::shared_inbox)
        .service(users::users_router())
        .service(posts::posts_router())
}
