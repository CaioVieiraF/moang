use actix_session::Session;
use actix_web::{get, http::header, web::Json, web::Path, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    establish_connection,
    models::{Post, RemoteActor, RemotePost},
    routes::{
        activity_pub::{
            content_object::ContentObject, remote_note::RemoteNote, Context,
        },
        users::{has_user, timeline::RemotePostView},
    },
    schema::{posts, remote_actors, remote_interactions, remote_posts},
    user_is_loged_in,
};

const ACTIVITY_JSON: &str = "application/activity+json";

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
                    .content_type(ACTIVITY_JSON)
                    .json(Json(object)),
                Err(error) => HttpResponse::InternalServerError().body(error),
            }
        }
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/posts/{ap_id}/likes")]
pub async fn get_likes(path: Path<(String, String)>) -> HttpResponse {
    get_reactions(path, "like", "likes").await
}

#[get("/posts/{ap_id}/shares")]
pub async fn get_shares(path: Path<(String, String)>) -> HttpResponse {
    get_reactions(path, "repost", "shares").await
}

async fn get_reactions(
    path: Path<(String, String)>,
    interaction_type: &str,
    collection_name: &str,
) -> HttpResponse {
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
    let collection_url = format!("{object_url}/{collection_name}");
    let actors = remote_interactions::table
        .filter(remote_interactions::object_id.eq(&object_url))
        .filter(remote_interactions::interaction_type.eq(interaction_type))
        .select(remote_interactions::actor_id)
        .order(remote_interactions::created_at.desc())
        .load::<String>(connection);

    match actors {
        Ok(actors) => HttpResponse::Ok()
            .content_type(ACTIVITY_JSON)
            .json(Json(json!({
                "@context": Context::activity_streams(),
                "id": collection_url,
                "type": "OrderedCollection",
                "totalItems": actors.len(),
                "orderedItems": actors,
            }))),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

/// The frontend reads a compact JSON shape from this URL, while other
/// ActivityPub servers expect a real `OrderedCollection` of `Note` objects.
/// Both are served from the same id, chosen by the `Accept` header.
#[get("/posts/{ap_id}/replies")]
pub async fn get_replies(req: HttpRequest, path: Path<(String, String)>) -> HttpResponse {
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
            if wants_activity_pub(&req) {
                let notes = rows
                    .iter()
                    .map(|(post, _)| RemoteNote::new(post))
                    .collect::<Vec<RemoteNote>>();

                // `orderedItems` works for generic AS2 consumers; the embedded
                // `first` page is what Mastodon and friends look for.
                HttpResponse::Ok()
                    .content_type(ACTIVITY_JSON)
                    .json(Json(json!({
                        "@context": Context::activity_streams(),
                        "id": replies_url,
                        "type": "OrderedCollection",
                        "totalItems": notes.len(),
                        "orderedItems": notes,
                        "first": {
                            "id": format!("{replies_url}?page=1"),
                            "type": "OrderedCollectionPage",
                            "partOf": replies_url,
                            "orderedItems": notes,
                        }
                    })))
            } else {
                let ordered_items = rows
                    .iter()
                    .map(|(post, actor)| RemotePostView::new(post, actor))
                    .collect::<Vec<RemotePostView>>();

                HttpResponse::Ok()
                    .content_type("application/json")
                    .json(Json(json!({
                        "id": replies_url,
                        "type": "OrderedCollection",
                        "totalItems": ordered_items.len(),
                        "orderedItems": ordered_items,
                    })))
            }
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

fn wants_activity_pub(req: &HttpRequest) -> bool {
    req.headers()
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .map(|accept| {
            accept.contains("application/activity+json") || accept.contains("application/ld+json")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::wants_activity_pub;
    use actix_web::{http::header, test::TestRequest};

    fn request(accept: Option<&str>) -> actix_web::HttpRequest {
        let mut request = TestRequest::default();
        if let Some(accept) = accept {
            request = request.insert_header((header::ACCEPT, accept));
        }
        request.to_http_request()
    }

    #[test]
    fn activity_pub_clients_get_the_as2_collection() {
        assert!(wants_activity_pub(&request(Some(
            "application/activity+json"
        ))));
        assert!(wants_activity_pub(&request(Some(
            "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\""
        ))));
        assert!(wants_activity_pub(&request(Some(
            "application/activity+json, application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\""
        ))));
    }

    #[test]
    fn browsers_and_the_frontend_get_the_application_json() {
        assert!(!wants_activity_pub(&request(Some(
            "application/json, text/plain, */*"
        ))));
        assert!(!wants_activity_pub(&request(Some(
            "text/html,application/xhtml+xml"
        ))));
        assert!(!wants_activity_pub(&request(Some("*/*"))));
        assert!(!wants_activity_pub(&request(None)));
    }
}
