use actix_web::{get, web::Json, web::Path, HttpResponse};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    establish_connection,
    models::{RemoteActor, RemotePost, User},
    routes::users::has_user,
    schema::{following, posts, remote_actors, remote_posts},
};

#[derive(Serialize)]
pub struct RemoteActorView {
    pub id: String,
    pub username: String,
    pub name: String,
    pub domain: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

#[derive(Serialize)]
pub struct RemotePostView {
    pub id: String,
    pub url: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub published: String,
    pub updated: Option<String>,
    pub sensitive: bool,
    #[serde(rename = "inReplyTo")]
    pub in_reply_to: Option<String>,
    pub actor: RemoteActorView,
}

impl RemotePostView {
    pub fn new(post: &RemotePost, actor: &RemoteActor) -> RemotePostView {
        let published: DateTime<Utc> = post.published.into();

        RemotePostView {
            id: post.id.clone(),
            url: post.url.clone(),
            content: post.content.clone(),
            summary: post.summary.clone(),
            published: format!("{}", published.format("%+")),
            updated: post.updated_at.map(|value| {
                let updated: DateTime<Utc> = value.into();
                format!("{}", updated.format("%+"))
            }),
            sensitive: post.sensitive,
            in_reply_to: post.in_reply_to.clone(),
            actor: RemoteActorView {
                id: actor.id.clone(),
                username: actor.preferred_username.clone(),
                name: actor.name.clone(),
                domain: actor.domain.clone(),
                avatar_url: actor.avatar_url.clone(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct Timeline {
    #[serde(rename = "totalItems")]
    pub total_items: usize,

    #[serde(rename = "orderedItems")]
    pub ordered_items: Vec<RemotePostView>,
}

#[get("/timeline")]
pub async fn get_timeline(path: Path<String>) -> HttpResponse {
    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let connection = &mut establish_connection();
    let followed = match followed_actors(&user, connection) {
        Ok(followed) => followed,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };
    let local_objects = match local_object_urls(&user, &username, connection) {
        Ok(objects) => objects,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    let rows = remote_posts::table
        .inner_join(remote_actors::table)
        .filter(
            remote_posts::actor_id
                .eq_any(&followed)
                .or(remote_posts::in_reply_to.eq_any(&local_objects)),
        )
        .order(remote_posts::published.desc())
        .limit(100)
        .select((RemotePost::as_select(), RemoteActor::as_select()))
        .load::<(RemotePost, RemoteActor)>(connection);

    match rows {
        Ok(rows) => {
            let ordered_items = rows
                .iter()
                .map(|(post, actor)| RemotePostView::new(post, actor))
                .collect::<Vec<RemotePostView>>();

            HttpResponse::Ok().json(Json(Timeline {
                total_items: ordered_items.len(),
                ordered_items,
            }))
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

fn followed_actors(
    user: &User,
    connection: &mut diesel::PgConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    following::table
        .filter(following::user_id.eq(&user.id))
        .filter(following::accepted.eq(true))
        .select(following::actor_id)
        .load::<String>(connection)
}

fn local_object_urls(
    user: &User,
    username: &str,
    connection: &mut diesel::PgConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    let config = AppConfig::get();
    let ids = posts::table
        .filter(posts::author.eq(&user.id))
        .select(posts::ap_id)
        .load::<Uuid>(connection)?;

    Ok(ids
        .iter()
        .map(|ap_id| config.object_url(username, ap_id))
        .collect())
}
