use std::time::SystemTime;

use actix_web::{
    post,
    web::{Bytes, Path},
    HttpRequest, HttpResponse,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    establish_connection,
    models::{RemoteActor, RemotePost, User},
    routes::{
        activity_pub::{
            delivery::{enqueue_activity, new_activity_id, save_activity},
            fetch::{fetch_remote_actor, validate_remote_url},
            signature::{signature_key_id, verify_request},
            Context, ObjType,
        },
        users::has_user,
    },
    schema::{activities, followers, following, posts, remote_actors, remote_posts, users},
};

const MAX_BODY_SIZE: usize = 1_048_576;

#[post("/inbox")]
pub async fn shared_inbox(req: HttpRequest, body: Bytes) -> HttpResponse {
    handle_inbox(req, body, None).await
}

#[post("")]
pub async fn personal_inbox(
    req: HttpRequest,
    path: Path<String>,
    body: Bytes,
) -> HttpResponse {
    handle_inbox(req, body, Some(path.into_inner().to_lowercase())).await
}

async fn handle_inbox(req: HttpRequest, body: Bytes, username: Option<String>) -> HttpResponse {
    if body.len() > MAX_BODY_SIZE {
        return HttpResponse::PayloadTooLarge().finish();
    }

    let payload: Value = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let Some(activity_type) = payload
        .get("type")
        .and_then(Value::as_str)
        .map(str::to_string)
    else {
        return HttpResponse::BadRequest().finish();
    };
    let Some(activity_id) = id_of(&payload) else {
        return HttpResponse::BadRequest().finish();
    };
    let Some(actor_id) = payload.get("actor").and_then(id_of) else {
        return HttpResponse::BadRequest().finish();
    };
    if validate_remote_url(&actor_id).is_err() {
        return HttpResponse::BadRequest().finish();
    }

    let headers = req.headers().clone();
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|value| value.as_str())
        .unwrap_or("/");

    let mut remote_actor = match fetch_remote_actor(&actor_id, false).await {
        Ok(actor) => actor,
        Err(error) => {
            log::warn!("could not fetch actor {actor_id}: {error}");
            return HttpResponse::Unauthorized().finish();
        }
    };

    if let Some(key_id) = signature_key_id(&headers) {
        if key_id != remote_actor.public_key_id {
            remote_actor = match fetch_remote_actor(&actor_id, true).await {
                Ok(actor) => actor,
                Err(error) => {
                    log::warn!("could not refresh actor {actor_id}: {error}");
                    return HttpResponse::Unauthorized().finish();
                }
            };
        }
    }

    if let Err(error) = verify_request("POST", path_and_query, &headers, &body, &remote_actor) {
        log::warn!("invalid signature from {actor_id}: {error}");
        return HttpResponse::Unauthorized().finish();
    }

    if activity_exists(&activity_id) {
        return HttpResponse::Accepted().finish();
    }

    let Some(local_user) = local_recipient(&payload, username.as_deref(), &remote_actor) else {
        return HttpResponse::Accepted().finish();
    };

    let result = process_activity(&payload, &activity_type, &local_user, &remote_actor);
    if let Err(error) = result {
        log::error!("error processing {activity_type} {activity_id}: {error}");
        return HttpResponse::InternalServerError().finish();
    }

    let object_id = payload.get("object").and_then(id_of);
    if let Err(error) = save_activity(
        &activity_id,
        Some(&local_user.id),
        &remote_actor.id,
        &activity_type,
        object_id.as_deref(),
        "in",
        payload,
    ) {
        log::error!("could not store inbound activity {activity_id}: {error}");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Accepted().finish()
}

fn process_activity(
    payload: &Value,
    activity_type: &str,
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    match activity_type {
        "Follow" => handle_follow(payload, local_user, remote_actor),
        "Undo" => handle_undo(local_user, remote_actor),
        "Accept" => set_following_accepted(local_user, remote_actor, true),
        "Reject" => remove_following(local_user, remote_actor),
        "Create" => handle_create(payload, remote_actor),
        "Update" => handle_update(payload, remote_actor),
        "Delete" => handle_delete(payload, remote_actor),
        "Block" => remove_follower_and_following(local_user, remote_actor),
        "Announce" | "Like" | "Flag" => Ok(()),
        _ => Ok(()),
    }
}

fn handle_follow(
    payload: &Value,
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    let config = AppConfig::get();
    let object_id = payload.get("object").and_then(id_of).unwrap_or_default();
    if object_id != config.actor_url(&local_user.name) {
        return Err(format!(
            "Follow object {object_id} is not {}",
            config.actor_url(&local_user.name)
        ));
    }

    let accepted = !local_user.is_locked;
    let connection = &mut establish_connection();
    diesel::insert_into(followers::table)
        .values((
            followers::user_id.eq(&local_user.id),
            followers::actor_id.eq(&remote_actor.id),
            followers::accepted.eq(accepted),
        ))
        .on_conflict((followers::user_id, followers::actor_id))
        .do_update()
        .set(followers::accepted.eq(accepted))
        .execute(connection)
        .map_err(|error| error.to_string())?;

    if accepted {
        send_accept(payload, local_user, remote_actor)?;
    }

    Ok(())
}

fn send_accept(
    follow: &Value,
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    let config = AppConfig::get();
    let (activity_id, _) = new_activity_id(&local_user.name);
    let follow_id = id_of(follow).unwrap_or_default();
    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": ObjType::Accept,
        "actor": config.actor_url(&local_user.name),
        "object": follow,
        "to": [remote_actor.id],
    });

    save_activity(
        &activity_id,
        Some(&local_user.id),
        &config.actor_url(&local_user.name),
        "Accept",
        Some(&follow_id),
        "out",
        payload,
    )?;
    enqueue_activity(&activity_id, std::slice::from_ref(&remote_actor.inbox_url))
}

fn handle_undo(local_user: &User, remote_actor: &RemoteActor) -> Result<(), String> {
    let connection = &mut establish_connection();
    diesel::delete(
        followers::table
            .filter(followers::user_id.eq(&local_user.id))
            .filter(followers::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn set_following_accepted(
    local_user: &User,
    remote_actor: &RemoteActor,
    accepted: bool,
) -> Result<(), String> {
    let connection = &mut establish_connection();
    diesel::update(
        following::table
            .filter(following::user_id.eq(&local_user.id))
            .filter(following::actor_id.eq(&remote_actor.id)),
    )
    .set(following::accepted.eq(accepted))
    .execute(connection)
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn remove_following(local_user: &User, remote_actor: &RemoteActor) -> Result<(), String> {
    let connection = &mut establish_connection();
    diesel::delete(
        following::table
            .filter(following::user_id.eq(&local_user.id))
            .filter(following::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn remove_follower_and_following(
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    let connection = &mut establish_connection();
    diesel::delete(
        followers::table
            .filter(followers::user_id.eq(&local_user.id))
            .filter(followers::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    .map_err(|error| error.to_string())?;
    diesel::delete(
        following::table
            .filter(following::user_id.eq(&local_user.id))
            .filter(following::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn handle_create(payload: &Value, remote_actor: &RemoteActor) -> Result<(), String> {
    let Some(object) = payload.get("object") else {
        return Ok(());
    };
    if object.is_string() {
        return Ok(());
    }

    save_remote_post(object, remote_actor)
}

fn handle_update(payload: &Value, remote_actor: &RemoteActor) -> Result<(), String> {
    let Some(object) = payload.get("object") else {
        return Ok(());
    };

    let object_id = id_of(object);
    if object_id.as_deref() == Some(remote_actor.id.as_str()) {
        let _ = fetch_remote_actor(&remote_actor.id, true);
        return Ok(());
    }

    save_remote_post(object, remote_actor)
}

fn handle_delete(payload: &Value, remote_actor: &RemoteActor) -> Result<(), String> {
    let Some(object_id) = payload.get("object").and_then(id_of) else {
        return Ok(());
    };

    let connection = &mut establish_connection();
    if object_id == remote_actor.id {
        diesel::delete(remote_actors::table.find(&remote_actor.id))
            .execute(connection)
            .map_err(|error| error.to_string())?;
        return Ok(());
    }

    diesel::delete(
        remote_posts::table
            .filter(remote_posts::id.eq(&object_id))
            .filter(remote_posts::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn save_remote_post(object: &Value, remote_actor: &RemoteActor) -> Result<(), String> {
    let Some(id) = id_of(object) else {
        return Ok(());
    };

    let raw_content = object
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let content = ammonia::clean(raw_content);
    let summary = object
        .get("summary")
        .and_then(Value::as_str)
        .map(ammonia::clean);
    let published = object
        .get("published")
        .and_then(Value::as_str)
        .and_then(parse_time)
        .unwrap_or_else(SystemTime::now);
    let updated_at = object
        .get("updated")
        .and_then(Value::as_str)
        .and_then(parse_time);
    let url = object
        .get("url")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| Some(id.clone()));
    let in_reply_to = object.get("inReplyTo").and_then(id_of);
    let sensitive = object
        .get("sensitive")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let remote_post = RemotePost {
        id: id.clone(),
        actor_id: remote_actor.id.clone(),
        content,
        summary,
        published,
        updated_at,
        url,
        in_reply_to,
        sensitive,
        created_at: SystemTime::now(),
    };

    let connection = &mut establish_connection();
    let updated_rows = diesel::update(remote_posts::table.find(&id))
        .set((
            remote_posts::content.eq(&remote_post.content),
            remote_posts::summary.eq(&remote_post.summary),
            remote_posts::updated_at.eq(remote_post.updated_at),
            remote_posts::url.eq(&remote_post.url),
            remote_posts::in_reply_to.eq(&remote_post.in_reply_to),
            remote_posts::sensitive.eq(remote_post.sensitive),
        ))
        .execute(connection)
        .map_err(|error| error.to_string())?;

    if updated_rows == 0 {
        diesel::insert_into(remote_posts::table)
            .values(&remote_post)
            .execute(connection)
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn local_recipient(
    payload: &Value,
    username: Option<&str>,
    remote_actor: &RemoteActor,
) -> Option<User> {
    if let Some(username) = username {
        return has_user(username.to_string());
    }

    let config = AppConfig::get();
    let connection = &mut establish_connection();
    let mut targets = ids_of(payload.get("to"));
    targets.extend(ids_of(payload.get("cc")));
    if let Some(object_id) = payload.get("object").and_then(id_of) {
        targets.push(object_id);
    }

    let local_users = users::table
        .select(User::as_select())
        .load::<User>(connection)
        .ok()?;

    for user in &local_users {
        if targets.iter().any(|target| {
            target == &config.actor_url(&user.name)
                || target == &config.followers_url(&user.name)
                || target == &config.inbox_url(&user.name)
        }) {
            return Some((*user).clone());
        }
    }

    let followed_by = following::table
        .filter(following::actor_id.eq(&remote_actor.id))
        .filter(following::accepted.eq(true))
        .select(following::user_id)
        .first::<String>(connection)
        .optional()
        .ok()?;

    if let Some(user_id) = followed_by {
        if let Ok(user) = users::table
            .find(user_id)
            .select(User::as_select())
            .first::<User>(connection)
        {
            return Some(user);
        }
    }

    let reply_target = payload
        .get("object")
        .and_then(|object| object.get("inReplyTo"))
        .and_then(id_of);
    if let Some(reply_target) = reply_target {
        if let Some(user) = local_post_author(&reply_target) {
            return Some(user);
        }
    }

    let _ = remote_actor;
    None
}

fn local_post_author(object_url: &str) -> Option<User> {
    let ap_id = object_url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .and_then(|value| Uuid::parse_str(value).ok())?;

    let connection = &mut establish_connection();
    let author_id = posts::table
        .filter(posts::ap_id.eq(ap_id))
        .select(posts::author)
        .first::<String>(connection)
        .optional()
        .ok()??;

    users::table
        .find(author_id)
        .select(User::as_select())
        .first::<User>(connection)
        .ok()
}

fn activity_exists(activity_id: &str) -> bool {
    let connection = &mut establish_connection();
    activities::table
        .find(activity_id)
        .select(activities::id)
        .first::<String>(connection)
        .optional()
        .ok()
        .flatten()
        .is_some()
}

fn id_of(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_string)
        .or_else(|| value.get("id").and_then(Value::as_str).map(str::to_string))
}

fn ids_of(value: Option<&Value>) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };

    if let Some(array) = value.as_array() {
        return array.iter().filter_map(id_of).collect();
    }

    id_of(value).into_iter().collect()
}

fn parse_time(value: &str) -> Option<SystemTime> {
    let parsed = DateTime::parse_from_rfc3339(value).ok()?;
    Some(SystemTime::from(parsed.with_timezone(&Utc)))
}
