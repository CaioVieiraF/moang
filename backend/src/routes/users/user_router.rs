use actix_web::{web, Scope};

use super::{get_user, inbox, outbox};

pub fn user_router() -> Scope {
    web::scope("{user_name}")
        .service(get_user::get_user)
        .service(outbox::outbox_router())
        .service(inbox::inbox_router())
}
