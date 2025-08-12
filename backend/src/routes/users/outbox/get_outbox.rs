use crate::{
    establish_connection,
    routes::{activity_pub::collection::OrderedCollectionBuilder, users::has_user},
};
use actix_web::{
    get,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};
use diesel::prelude::*;

#[get("")]
pub async fn get_outbox(path: Path<String>, req: HttpRequest) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    if has_user(path.into_inner()).is_none() {
        return HttpResponse::NotFound().finish();
    }

    let connection = &mut establish_connection();
    let query_result = posts.count().get_result(connection);

    match query_result {
        Ok(total_items) => {
            let all_posts = OrderedCollectionBuilder::new()
                .id(req.full_url())
                .total_items(total_items)
                .first("posts")
                .build();

            HttpResponse::Ok()
                .content_type(
                    "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                )
                .json(Json(all_posts))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Server error: {e}")),
    }
}
