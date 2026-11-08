pub mod config;
pub mod models;
pub mod routes;
pub mod schema;

use core::panic;
use std::env;

use actix_session::Session;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, Validation};
use models::Claims;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations() {
    let connection = &mut establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running database migrations");
}

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

/// Email stored in the session token. Used by actions that need to know
/// *which* user is logged in, not only that somebody is.
pub fn current_user_email(session: &Session) -> Option<String> {
    let token = session.get::<String>("user_identity").ok()??;
    let secret = env::var("JWT_HASH").ok()?;
    let claims = jsonwebtoken::decode::<Claims>(
        token.trim_start_matches("Bearer "),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .ok()?
    .claims;

    Some(claims.sub)
}

/// Id of the logged-in user, ready to be compared with `posts.author` or
/// `users.id`. Ownership checks must use this instead of trusting the URL.
pub fn current_user_id(session: &Session) -> Option<String> {
    use crate::schema::users;

    let email = current_user_email(session)?;
    let connection = &mut establish_connection();

    users::table
        .filter(users::email.eq(&email))
        .select(users::id)
        .first::<String>(connection)
        .ok()
}
