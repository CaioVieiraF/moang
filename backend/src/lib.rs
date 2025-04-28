pub mod models;
pub mod routes;
pub mod schema;

use core::panic;
use std::env;

use actix_session::Session;
use actix_web::http::header::{HeaderMap, HeaderValue};
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, Validation};
use models::Claims;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn validate_user(auth_token: String) -> Option<bool> {
    let secret = env::var("JWT_HASH").expect("JWT_HASH missing!");

    let is_user_valid = jsonwebtoken::decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .is_ok();
    Some(is_user_valid)
}

fn user_is_loged_in(session: &Session) -> bool {
    if let Ok(Some(auth_header)) = session.get("user_identity") {
        validate_user(auth_header).unwrap_or(false)
    } else {
        false
    }
}
