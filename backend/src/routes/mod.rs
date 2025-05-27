mod login;
mod posts;
mod users;
use actix_web::{
    web::{self},
    Scope,
};

pub fn router() -> Scope {
    web::scope("")
        .service(posts::posts_router())
        .service(users::users_router())
        .service(login::login)
}
