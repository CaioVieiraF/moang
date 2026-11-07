use actix_web::{web, Scope};

use crate::routes::activity_pub::inbox::personal_inbox;

pub fn inbox_router() -> Scope {
    web::scope("inbox").service(personal_inbox)
}
