pub mod models;
pub mod routes;
pub mod schema;

use core::panic;
use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
