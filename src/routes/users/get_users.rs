use crate::{establish_connection, models::User};
use actix_web::{get, web::Json, HttpResponse};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

#[get("")]
pub async fn get_users() -> HttpResponse {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let query_result = users.select(User::as_select()).load(connection);

    match query_result {
        Ok(retreived_users) => {
            let user_names = retreived_users
                .iter()
                .map(|user| user.name.to_owned())
                .collect::<Vec<String>>();
            HttpResponse::Ok().json(Json(user_names))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
