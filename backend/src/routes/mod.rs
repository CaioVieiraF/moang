mod activity_pub;
mod login;
mod posts;
mod users;
use actix_web::{
    web::{self},
    Scope,
};

pub fn router() -> Scope {
    web::scope("")
        .service(login::login)
        .service(users::users_router())
        .service(posts::posts_router())
}
