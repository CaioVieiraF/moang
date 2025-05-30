mod create_user;
mod delete_user;
mod get_user;
mod inbox;
mod outbox;
mod user_router;

use actix_web::{web, Scope};

use crate::{establish_connection, models::User};

pub fn has_user(user_name: String) -> Option<User> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();
    let query_result = users
        .select(User::as_select())
        .filter(name.eq(user_name))
        .first(connection);

    query_result.ok()
}

pub fn users_router() -> Scope {
    web::scope("users")
        .service(user_router::user_router())
        .service(create_user::create_user)
        .service(delete_user::delete_user)
}
