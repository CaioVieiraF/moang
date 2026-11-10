use actix_session::Session;
use actix_web::{
    get,
    post,
    web::{Json, Path},
    HttpResponse,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    establish_connection,
    models::{RemoteActor, User},
    routes::{
        activity_pub::{delivery::backfill_for_follower, inbox::send_follow_response},
        users::{following::owns_account, has_user},
    },
    schema::{activities, followers, remote_actors},
    user_is_loged_in,
};

#[derive(Deserialize)]
pub struct FollowRequestBody {
    actor: String,
}

#[derive(Serialize)]
pub struct FollowRequestView {
    pub id: String,
    pub username: String,
    pub name: String,
    pub domain: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

#[get("/follow-requests")]
pub async fn list_follow_requests(session: Session, path: Path<String>) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username) else {
        return HttpResponse::NotFound().finish();
    };
    if !owns_account(&session, &user) {
        return HttpResponse::Forbidden().finish();
    }

    let connection = &mut establish_connection();
    let rows = followers::table
        .inner_join(remote_actors::table)
        .filter(followers::user_id.eq(&user.id))
        .filter(followers::accepted.eq(false))
        .order(followers::created_at.asc())
        .select(RemoteActor::as_select())
        .load::<RemoteActor>(connection);

    match rows {
        Ok(actors) => HttpResponse::Ok().json(Json(
            actors
                .into_iter()
                .map(|actor| FollowRequestView {
                    id: actor.id,
                    username: actor.preferred_username,
                    name: actor.name,
                    domain: actor.domain,
                    avatar_url: actor.avatar_url,
                })
                .collect::<Vec<FollowRequestView>>(),
        )),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/follow-requests/accept")]
pub async fn accept_follow_request(
    session: Session,
    path: Path<String>,
    request: Json<FollowRequestBody>,
) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username) else {
        return HttpResponse::NotFound().finish();
    };
    if !owns_account(&session, &user) {
        return HttpResponse::Forbidden().finish();
    }

    let connection = &mut establish_connection();
    let Some(remote_actor) = pending_request(connection, &user, &request.actor) else {
        return HttpResponse::NotFound().finish();
    };
    let Some(follow) = stored_follow(connection, &user, &remote_actor.id) else {
        return HttpResponse::Conflict().body("Follow activity not found");
    };

    if let Err(error) = diesel::update(
        followers::table
            .filter(followers::user_id.eq(&user.id))
            .filter(followers::actor_id.eq(&remote_actor.id)),
    )
    .set(followers::accepted.eq(true))
    .execute(connection)
    {
        return HttpResponse::InternalServerError().body(error.to_string());
    }

    if let Err(error) = send_follow_response(&follow, "Accept", &user, &remote_actor) {
        return HttpResponse::InternalServerError().body(error);
    }
    if let Err(error) = backfill_for_follower(&user, &remote_actor.inbox_url) {
        log::warn!("could not backfill posts for {}: {error}", remote_actor.id);
    }

    HttpResponse::NoContent().finish()
}

#[post("/follow-requests/reject")]
pub async fn reject_follow_request(
    session: Session,
    path: Path<String>,
    request: Json<FollowRequestBody>,
) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username) else {
        return HttpResponse::NotFound().finish();
    };
    if !owns_account(&session, &user) {
        return HttpResponse::Forbidden().finish();
    }

    let connection = &mut establish_connection();
    let Some(remote_actor) = pending_request(connection, &user, &request.actor) else {
        return HttpResponse::NotFound().finish();
    };
    let follow = stored_follow(connection, &user, &remote_actor.id);

    if let Err(error) = diesel::delete(
        followers::table
            .filter(followers::user_id.eq(&user.id))
            .filter(followers::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    {
        return HttpResponse::InternalServerError().body(error.to_string());
    }

    if let Some(follow) = follow {
        if let Err(error) = send_follow_response(&follow, "Reject", &user, &remote_actor) {
            return HttpResponse::InternalServerError().body(error);
        }
    }

    HttpResponse::NoContent().finish()
}

fn pending_request(
    connection: &mut diesel::PgConnection,
    user: &User,
    actor_id: &str,
) -> Option<RemoteActor> {
    followers::table
        .inner_join(remote_actors::table)
        .filter(followers::user_id.eq(&user.id))
        .filter(followers::actor_id.eq(actor_id))
        .filter(followers::accepted.eq(false))
        .select(RemoteActor::as_select())
        .first::<RemoteActor>(connection)
        .optional()
        .ok()
        .flatten()
}

fn stored_follow(
    connection: &mut diesel::PgConnection,
    user: &User,
    actor_id: &str,
) -> Option<Value> {
    activities::table
        .filter(activities::user_id.eq(&user.id))
        .filter(activities::activity_type.eq("Follow"))
        .filter(activities::actor_id.eq(actor_id))
        .filter(activities::direction.eq("in"))
        .order(activities::created_at.desc())
        .select(activities::payload)
        .first::<Value>(connection)
        .optional()
        .ok()
        .flatten()
}
