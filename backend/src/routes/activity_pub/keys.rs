use std::time::SystemTime;

use diesel::prelude::*;
use openssl::rsa::Rsa;

use crate::{
    establish_connection,
    models::ActorKey,
    schema::actor_keys,
};

/// Returns the key pair for an actor, generating one on demand.
///
/// Generating on demand keeps the first run simple for existing users. New users
/// get a key during registration, so this is mostly a safety net.
pub fn get_or_create_key(user_id: &str) -> Result<ActorKey, String> {
    let connection = &mut establish_connection();

    if let Some(key) = actor_keys::table
        .filter(actor_keys::user_id.eq(user_id))
        .select(ActorKey::as_select())
        .first(connection)
        .optional()
        .map_err(|error| error.to_string())?
    {
        return Ok(key);
    }

    let rsa = Rsa::generate(2048).map_err(|error| error.to_string())?;
    let private_key_pem = String::from_utf8(rsa.private_key_to_pem().map_err(|e| e.to_string())?)
        .map_err(|error| error.to_string())?;
    let public_key_pem = String::from_utf8(rsa.public_key_to_pem().map_err(|e| e.to_string())?)
        .map_err(|error| error.to_string())?;

    let key = ActorKey {
        user_id: user_id.to_string(),
        public_key_pem,
        private_key_pem,
        created_at: SystemTime::now(),
    };

    diesel::insert_into(actor_keys::table)
        .values(&key)
        .on_conflict_do_nothing()
        .execute(connection)
        .map_err(|error| error.to_string())?;

    // Another request may have inserted the key between our select and insert.
    actor_keys::table
        .filter(actor_keys::user_id.eq(user_id))
        .select(ActorKey::as_select())
        .first(connection)
        .map_err(|error| error.to_string())
}
