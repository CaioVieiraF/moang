mod status;

use actix_web::{web, Scope};
use status::get_status;

pub fn router() -> Scope {
    web::scope("").service(get_status)
}
