use actix_web::{web, Scope};

pub fn inbox_router() -> Scope {
    web::scope("inbox")
}
