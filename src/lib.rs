pub mod models;
pub mod routes;
pub mod schema;

use core::panic;
use std::env;

use actix_web::{
    http::header::{HeaderMap, HeaderValue},
    HttpResponse,
};
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

fn validate_user(auth_header: &HeaderValue) -> Option<bool> {
    if let Ok(auth_token) = auth_header.to_str() {
        if auth_token.starts_with("Bearer ") {
            let token = auth_token.trim_start_matches("Bearer ");
            let secret = env::var("JWT_HASH").expect("JWT_HASH missing!");

            if jsonwebtoken::decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .is_ok()
            {
                return Some(true);
            } else {
                return Some(false);
            }
        }
    }

    None
}

fn user_is_loged_in(headers: &HeaderMap) -> bool {
    if let Some(auth_header) = headers.get("Authorization") {
        validate_user(auth_header).unwrap_or(false)
    } else {
        false
    }
}
