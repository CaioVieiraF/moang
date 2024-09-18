use actix_web::{options, post, web::Json, HttpResponse};
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
    let new_user = new_user.into_inner();

    let connection = &mut establish_connection();
    let query_result = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .execute(connection);

    match query_result {
        Ok(_) => match new_user.validate() {
            Ok(_) => HttpResponse::Created().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[options("")]
pub async fn validate_create_user(new_user: Json<NewUser>) -> HttpResponse {
    let new_user = new_user.into_inner();

    if new_user.email.len() > 1 {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
