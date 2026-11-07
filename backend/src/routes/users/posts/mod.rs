use actix_session::Session;
use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    establish_connection,
    models::{Post, RemoteActor, RemotePost},
    routes::{
        activity_pub::content_object::ContentObject,
        users::{has_user, timeline::RemotePostView},
    },
    schema::{posts, remote_actors, remote_posts},
    user_is_loged_in,
};

#[get("/posts/{ap_id}")]
pub async fn get_object(session: Session, path: Path<(String, String)>) -> HttpResponse {
    let (username, ap_id) = path.into_inner();
    let username = username.to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let Ok(ap_id) = Uuid::parse_str(&ap_id) else {
        return HttpResponse::NotFound().finish();
    };

    let connection = &mut establish_connection();
    let post = posts::table
        .filter(posts::author.eq(&user.id))
        .filter(posts::ap_id.eq(ap_id))
        .select(Post::as_select())
        .first::<Post>(connection)
        .optional();

    match post {
        Ok(Some(post))
            if post.visibility != "private" || user_is_loged_in(&session) =>
        {
            match ContentObject::from_post(&post, &username) {
                Ok(object) => HttpResponse::Ok()
                    .content_type(
                        "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                    )
                    .json(Json(object)),
                Err(error) => HttpResponse::InternalServerError().body(error),
            }
        }
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/posts/{ap_id}/replies")]
pub async fn get_replies(path: Path<(String, String)>) -> HttpResponse {
    let (username, ap_id) = path.into_inner();
    let username = username.to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let Ok(ap_id) = Uuid::parse_str(&ap_id) else {
        return HttpResponse::NotFound().finish();
    };

    let connection = &mut establish_connection();
    let post_exists = posts::table
        .filter(posts::author.eq(&user.id))
        .filter(posts::ap_id.eq(ap_id))
        .select(posts::id)
        .first::<i32>(connection)
        .optional();

    match post_exists {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    }

    let config = AppConfig::get();
    let object_url = config.object_url(&username, &ap_id);
    let replies_url = format!("{object_url}/replies");
    let rows = remote_posts::table
        .inner_join(remote_actors::table)
        .filter(remote_posts::in_reply_to.eq(&object_url))
        .order(remote_posts::published.asc())
        .select((RemotePost::as_select(), RemoteActor::as_select()))
        .load::<(RemotePost, RemoteActor)>(connection);

    match rows {
        Ok(rows) => {
            let ordered_items = rows
                .iter()
                .map(|(post, actor)| RemotePostView::new(post, actor))
                .collect::<Vec<RemotePostView>>();

            HttpResponse::Ok()
                .content_type(
                    "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                )
                .json(Json(json!({
                    "id": replies_url,
                    "type": "OrderedCollection",
                    "totalItems": ordered_items.len(),
                    "orderedItems": ordered_items,
                })))
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
