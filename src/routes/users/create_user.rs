use actix_web::{post, web::Json, HttpResponse};
use diesel::{prelude::Insertable, RunQueryDsl, SelectableHelper};
use serde::Deserialize;
use validator::Validate;

use crate::{establish_connection, models::User, schema::users};

#[derive(Deserialize, Insertable, Validate)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    password: String,
    #[validate(email)]
    email: String,
}

#[post("")]
pub async fn create_user(new_user: Json<NewUser>) -> HttpResponse {
    use crate::schema::users;
    let mut new_user = new_user.into_inner();

    if new_user.validate().is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let entropy = b"some_entropy_to_hash_the_password";
    let argon_config = argon2::Config::default();
    let hashed_password =
        argon2::hash_encoded(new_user.password.as_bytes(), entropy, &argon_config);

    if hashed_password.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    new_user.password = hashed_password.unwrap();

    let connection = &mut establish_connection();
    let query_result = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .execute(connection);

    if query_result.is_ok() {
        HttpResponse::Created().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
