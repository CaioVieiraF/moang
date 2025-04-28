use crate::{establish_connection, models::User};
use actix_web::{get, web::Json, HttpResponse};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use serde::Serialize;

#[derive(Serialize)]
struct UserInfo {
    name: String,
}

#[get("")]
pub async fn get_users() -> HttpResponse {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let query_result = users.select(User::as_select()).load(connection);

    match query_result {
        Ok(retreived_users) => {
            let user_names = retreived_users
                .iter()
                .map(|user| UserInfo {
                    name: user.name.to_owned(),
                })
                .collect::<Vec<UserInfo>>();
            HttpResponse::Ok().json(Json(user_names))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
