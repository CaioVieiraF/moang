mod create_user;
mod get_users;

use actix_web::{web, Scope};

pub fn users_router() -> Scope {
    web::scope("users")
        .service(get_users::get_users)
        .service(create_user::create_user)
        .service(create_user::validate_create_user)
}
