use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use diesel::{RunQueryDsl, SelectableHelper};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{establish_connection, models::User, user_is_loged_in};

#[derive(Deserialize, Validate)]
struct NewUserRequest {
    name: String,
    password: String,
    #[validate(email)]
    email: String,
}

#[post("")]
pub async fn create_user(request: HttpRequest, request_data: Json<NewUserRequest>) -> HttpResponse {
    use crate::schema::users;

    if !user_is_loged_in(request.headers()) {
        return HttpResponse::Unauthorized().finish();
    }

    let request_data_user = request_data.into_inner();

    if request_data_user.validate().is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let entropy = Uuid::new_v4();
    let argon_config = argon2::Config::default();
    let hashed_password = argon2::hash_encoded(
        request_data_user.password.as_bytes(),
        entropy.as_bytes(),
        &argon_config,
    );

    if hashed_password.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        name: request_data_user.name,
        email: request_data_user.email,
        password: hashed_password.unwrap(),
    };

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
