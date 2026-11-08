use crate::{
    config::AppConfig,
    establish_connection,
    models::Post,
    routes::{
        activity_pub::collection::{OrderedCollectionPageBuilder, OrderedItem},
        users::has_user,
    },
};
use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;

#[get("/posts")]
pub async fn get_posts(path: Path<String>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let config = AppConfig::get();
    let connection = &mut establish_connection();
    let total_items = match posts
        .filter(author.eq(&user.id))
        .filter(is_public.eq(true))
        .count()
        .get_result::<i64>(connection)
    {
        Ok(total) => total,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    let query_result = posts
        .filter(author.eq(&user.id))
        .filter(is_public.eq(true))
        .order(created_at.desc())
        .select(Post::as_select())
        .load(connection);

    match query_result {
        Ok(retrieved_posts) => {
            let ordered_items = retrieved_posts
                .iter()
                .filter_map(|post| OrderedItem::from_post(post, &username).ok())
                .collect::<Vec<OrderedItem>>();

            let all_posts = OrderedCollectionPageBuilder::new()
                .id(format!("{}/posts", config.outbox_url(&username)))
                .part_of(config.outbox_url(&username))
                .total_items(total_items)
                .ordered_items(ordered_items)
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
