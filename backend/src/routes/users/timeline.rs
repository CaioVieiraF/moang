use actix_web::{get, web::Json, web::Path, HttpResponse};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

use crate::{
    establish_connection,
    models::{RemoteActor, RemotePost},
    routes::users::has_user,
    schema::{remote_actors, remote_posts},
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
    if has_user(username).is_none() {
        return HttpResponse::NotFound().finish();
    }

    let connection = &mut establish_connection();
    let rows = remote_posts::table
        .inner_join(remote_actors::table)
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
