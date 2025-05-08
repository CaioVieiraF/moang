use crate::{establish_connection, user_is_loged_in};
use actix_session::Session;
use actix_web::{delete, web::Path, HttpResponse};
use diesel::prelude::*;

#[delete("/{user_id}")]
pub async fn delete_user(session: Session, path: Path<String>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let connection = &mut establish_connection();
    let user_id = path.into_inner();
    let query_result = diesel::delete(users.find(user_id)).execute(connection);

    if query_result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
