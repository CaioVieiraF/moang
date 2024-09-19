use crate::establish_connection;
use actix_web::{delete, web::Path, HttpResponse};
use diesel::prelude::*;

#[delete("/{user_id}")]
pub async fn delete_user(path: Path<i32>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let user_id = path.into_inner();
    let query_result = diesel::delete(users.find(user_id)).execute(connection);

    if query_result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
