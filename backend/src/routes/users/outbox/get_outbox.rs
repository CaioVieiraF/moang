use crate::{
    config::AppConfig,
    establish_connection,
    routes::{activity_pub::collection::OrderedCollectionBuilder, users::has_user},
};
use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;

#[get("")]
pub async fn get_outbox(path: Path<String>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let config = AppConfig::get();
    let connection = &mut establish_connection();
    let query_result = posts
        .filter(author.eq(&user.id))
        .filter(is_public)
        .count()
        .get_result(connection);

    match query_result {
        Ok(total_items) => {
            let all_posts = OrderedCollectionBuilder::new()
                .id(config.outbox_url(&username))
                .total_items(total_items)
                .first("posts?page=1")
                .build();

            HttpResponse::Ok()
                .content_type(
                    "application/activity+json",
                )
                .json(Json(all_posts))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Server error: {e}")),
    }
}
