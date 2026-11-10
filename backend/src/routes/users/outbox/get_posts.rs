use crate::{
    config::AppConfig,
    establish_connection,
    models::Post,
    routes::{
        activity_pub::collection::{OrderedCollectionPageBuilder, OrderedItem},
        pagination::{PageQuery, PAGE_SIZE},
        users::has_user,
    },
};
use actix_web::{
    get,
    web::{Json, Path, Query},
    HttpResponse,
};
use diesel::prelude::*;

#[get("/posts")]
pub async fn get_posts(path: Path<String>, query: Query<PageQuery>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let page = query.page();
    let offset = query.offset();

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
        .order((created_at.desc(), id.desc()))
        .offset(offset)
        .limit(PAGE_SIZE)
        .select(Post::as_select())
        .load(connection);

    match query_result {
        Ok(retrieved_posts) => {
            let ordered_items = retrieved_posts
                .iter()
                .filter_map(|post| OrderedItem::from_post(post, &username).ok())
                .collect::<Vec<OrderedItem>>();

            let collection_url = format!("{}/posts", config.outbox_url(&username));
            let page_url = format!("{collection_url}?page={page}");
            let next = (offset + PAGE_SIZE < total_items)
                .then(|| format!("{collection_url}?page={}", page + 1));
            let prev = (page > 1).then(|| format!("{collection_url}?page={}", page - 1));

            let mut page_builder = OrderedCollectionPageBuilder::new()
                .id(page_url)
                .part_of(config.outbox_url(&username))
                .total_items(total_items)
                .ordered_items(ordered_items);

            if let Some(next) = next {
                page_builder = page_builder.next(next);
            }
            if let Some(prev) = prev {
                page_builder = page_builder.prev(prev);
            }

            HttpResponse::Ok()
                .content_type("application/activity+json")
                .json(Json(page_builder.build()))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Server error: {e}")),
    }
}
