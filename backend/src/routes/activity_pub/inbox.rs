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
    models::{NewRemoteInteraction, RemoteActor, RemotePost, User},
    routes::{
        activity_pub::{
            delivery::{backfill_for_follower, enqueue_activity, new_activity_id, save_activity},
            fetch::{fetch_remote_actor, validate_remote_url},
            signature::{signature_key_id, verify_request},
            Context,
        },
        users::has_user,
    },
    schema::{
        activities, followers, following, posts, remote_actors, remote_interactions, remote_posts,
        users,
    },
};

const MAX_BODY_SIZE: usize = 1_048_576;

#[post("/inbox")]
pub async fn shared_inbox(req: HttpRequest, body: Bytes) -> HttpResponse {
    handle_inbox(req, body, None).await
}

#[post("")]
pub async fn personal_inbox(req: HttpRequest, path: Path<String>, body: Bytes) -> HttpResponse {
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

    let Some(local_user) = local_recipient(&payload, username.as_deref()) else {
        return HttpResponse::Accepted().finish();
    };

    let mut remote_actor = match fetch_remote_actor(&actor_id, false, Some(&local_user)).await {
        Ok(actor) => actor,
        Err(error) => {
            log::warn!("could not fetch actor {actor_id}: {error}");
            return HttpResponse::Unauthorized().finish();
        }
    };

    if let Some(key_id) = signature_key_id(&headers) {
        if key_id != remote_actor.public_key_id {
            remote_actor = match fetch_remote_actor(&actor_id, true, Some(&local_user)).await {
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
        "Undo" => handle_undo(payload, local_user, remote_actor),
        "Accept" => set_following_accepted(local_user, remote_actor, true),
        "Reject" => remove_following(local_user, remote_actor),
        "Create" => handle_create(payload, remote_actor),
        "Update" => handle_update(payload, local_user, remote_actor),
        "Delete" => handle_delete(payload, remote_actor),
        "Block" => remove_follower_and_following(local_user, remote_actor),
        "Like" => handle_reaction(payload, remote_actor, "like"),
        "Announce" => handle_reaction(payload, remote_actor, "repost"),
        "Flag" => Ok(()),
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
        send_follow_response(payload, "Accept", local_user, remote_actor)?;
        match backfill_for_follower(local_user, &remote_actor.inbox_url) {
            Ok(count) => log::info!("backfilled {count} posts for {}", remote_actor.id),
            Err(error) => log::warn!("could not backfill posts for {}: {error}", remote_actor.id),
        }
    }

    Ok(())
}

pub(crate) fn send_follow_response(
    follow: &Value,
    response_type: &str,
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    let config = AppConfig::get();
    let (activity_id, _) = new_activity_id(&local_user.name);
    let follow_id = id_of(follow).unwrap_or_default();
    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": response_type,
        "actor": config.actor_url(&local_user.name),
        "object": follow,
        "to": [remote_actor.id],
    });

    save_activity(
        &activity_id,
        Some(&local_user.id),
        &config.actor_url(&local_user.name),
        response_type,
        Some(&follow_id),
        "out",
        payload,
    )?;
    enqueue_activity(&activity_id, std::slice::from_ref(&remote_actor.inbox_url))
}

fn handle_undo(
    payload: &Value,
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    if undo_targets_follow(payload, remote_actor) {
        let connection = &mut establish_connection();
        diesel::delete(
            followers::table
                .filter(followers::user_id.eq(&local_user.id))
                .filter(followers::actor_id.eq(&remote_actor.id)),
        )
        .execute(connection)
        .map_err(|error| error.to_string())?;

        return Ok(());
    }

    // Undoing a Like or Announce points at the original activity, so removing
    // the interaction by its id is enough.
    let Some(object_id) = undo_target_id(payload) else {
        return Ok(());
    };

    let connection = &mut establish_connection();
    diesel::delete(
        remote_interactions::table
            .filter(remote_interactions::id.eq(&object_id))
            .filter(remote_interactions::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection)
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn handle_reaction(
    payload: &Value,
    remote_actor: &RemoteActor,
    interaction_type: &str,
) -> Result<(), String> {
    let Some((activity_id, object_id)) = reaction_ids(payload) else {
        return Ok(());
    };

    // Reactions to remote posts are delivered to their own instance, so only
    // local objects are relevant to this blog.
    if local_post_author(&object_id).is_none() {
        return Ok(());
    }

    let interaction = NewRemoteInteraction {
        id: activity_id,
        actor_id: remote_actor.id.clone(),
        object_id,
        interaction_type: interaction_type.to_string(),
    };

    let connection = &mut establish_connection();
    diesel::insert_into(remote_interactions::table)
        .values(&interaction)
        .on_conflict_do_nothing()
        .execute(connection)
        .map_err(|error| error.to_string())?;

    Ok(())
}

fn undo_targets_follow(payload: &Value, remote_actor: &RemoteActor) -> bool {
    let Some(object) = payload.get("object") else {
        return false;
    };

    if object.get("type").and_then(Value::as_str) == Some("Follow") {
        return true;
    }

    let Some(object_id) = id_of(object) else {
        return false;
    };

    let connection = &mut establish_connection();
    activities::table
        .find(&object_id)
        .filter(activities::actor_id.eq(&remote_actor.id))
        .filter(activities::activity_type.eq("Follow"))
        .select(activities::id)
        .first::<String>(connection)
        .optional()
        .ok()
        .flatten()
        .is_some()
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

    save_remote_post(object, payload, remote_actor)
}

fn handle_update(
    payload: &Value,
    local_user: &User,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    let Some(object) = payload.get("object") else {
        return Ok(());
    };

    let object_id = id_of(object);
    if object_id.as_deref() == Some(remote_actor.id.as_str()) {
        let _ = fetch_remote_actor(&remote_actor.id, true, Some(local_user));
        return Ok(());
    }

    save_remote_post(object, payload, remote_actor)
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

fn save_remote_post(
    object: &Value,
    activity: &Value,
    remote_actor: &RemoteActor,
) -> Result<(), String> {
    let Some(id) = id_of(object) else {
        return Ok(());
    };

    // Some servers only put HTML in `contentMap`, and some only put the
    // audience on the wrapping activity instead of on the object.
    let raw_content = object
        .get("content")
        .and_then(Value::as_str)
        .or_else(|| {
            object
                .get("contentMap")
                .and_then(Value::as_object)
                .and_then(|map| map.values().find_map(Value::as_str))
        })
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
        .and_then(first_url)
        .unwrap_or_else(|| id.clone());
    // Servers disagree on the trailing slash; normalize it so replies always
    // match the canonical object id used by our own posts.
    let in_reply_to = object
        .get("inReplyTo")
        .and_then(id_of)
        .map(|value| value.trim_end_matches('/').to_string());
    let sensitive = object
        .get("sensitive")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let likes_count = collection_total(object.get("likes"));
    let reposts_count = collection_total(object.get("shares"));
    let replies_count = collection_total(object.get("replies"));
    let to_addresses = json!(ids_of(
        object.get("to").or_else(|| activity.get("to"))
    ));
    let cc_addresses = json!(ids_of(
        object.get("cc").or_else(|| activity.get("cc"))
    ));

    let connection = &mut establish_connection();

    // Updates may omit the interaction collections. Keep the previous counts
    // instead of resetting them to zero.
    let previous_counts = remote_posts::table
        .find(&id)
        .select((
            remote_posts::likes_count,
            remote_posts::reposts_count,
            remote_posts::replies_count,
        ))
        .first::<(i32, i32, i32)>(connection)
        .optional()
        .map_err(|error| error.to_string())?
        .unwrap_or((0, 0, 0));

    let remote_post = RemotePost {
        id: id.clone(),
        actor_id: remote_actor.id.clone(),
        content,
        summary,
        published,
        updated_at,
        url: Some(url),
        in_reply_to,
        sensitive,
        created_at: SystemTime::now(),
        likes_count: likes_count.unwrap_or(previous_counts.0),
        reposts_count: reposts_count.unwrap_or(previous_counts.1),
        replies_count: replies_count.unwrap_or(previous_counts.2),
        to_addresses,
        cc_addresses,
    };

    let updated_rows = diesel::update(remote_posts::table.find(&id))
        .set((
            remote_posts::content.eq(&remote_post.content),
            remote_posts::summary.eq(&remote_post.summary),
            remote_posts::updated_at.eq(remote_post.updated_at),
            remote_posts::url.eq(&remote_post.url),
            remote_posts::in_reply_to.eq(&remote_post.in_reply_to),
            remote_posts::sensitive.eq(&remote_post.sensitive),
            remote_posts::likes_count.eq(remote_post.likes_count),
            remote_posts::reposts_count.eq(remote_post.reposts_count),
            remote_posts::replies_count.eq(remote_post.replies_count),
            remote_posts::to_addresses.eq(&remote_post.to_addresses),
            remote_posts::cc_addresses.eq(&remote_post.cc_addresses),
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

fn local_recipient(payload: &Value, username: Option<&str>) -> Option<User> {
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

    let reply_target = payload
        .get("object")
        .and_then(|object| object.get("inReplyTo"))
        .and_then(id_of);
    if let Some(reply_target) = reply_target {
        if let Some(user) = local_post_author(&reply_target) {
            return Some(user);
        }
    }

    // Shared inboxes are per instance, so an activity sent here is for this
    // instance even when the recipients are not explicit. This also makes
    // Undo activities (which point at a Follow object instead of the actor)
    // resolve correctly.
    local_users.into_iter().next()
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

/// `url` can be a string, an object with `href`, or an array of either.
fn first_url(value: &Value) -> Option<String> {
    if let Some(text) = value.as_str() {
        return Some(text.to_string());
    }
    if let Some(href) = value.get("href").and_then(Value::as_str) {
        return Some(href.to_string());
    }

    value.as_array()?.iter().find_map(first_url)
}

fn collection_total(value: Option<&Value>) -> Option<i32> {
    value
        .and_then(|value| value.get("totalItems"))
        .and_then(Value::as_i64)
        .map(|total| total.clamp(0, i32::MAX as i64) as i32)
}

fn reaction_ids(payload: &Value) -> Option<(String, String)> {
    let activity_id = id_of(payload)?;
    let object_id = payload.get("object").and_then(id_of)?;

    Some((activity_id, object_id.trim_end_matches('/').to_string()))
}

fn undo_target_id(payload: &Value) -> Option<String> {
    payload.get("object").and_then(id_of)
}

fn parse_time(value: &str) -> Option<SystemTime> {
    let parsed = DateTime::parse_from_rfc3339(value).ok()?;
    Some(SystemTime::from(parsed.with_timezone(&Utc)))
}

#[cfg(test)]
mod tests {
    use super::{collection_total, first_url, id_of, ids_of, parse_time, reaction_ids, undo_target_id};
    use serde_json::json;

    #[test]
    fn reads_collection_totals() {
        let value = json!({ "type": "Collection", "totalItems": 42 });

        assert_eq!(collection_total(Some(&value)), Some(42));
        assert_eq!(
            collection_total(Some(&json!({ "totalItems": i64::MAX }))),
            Some(i32::MAX)
        );
    }

    #[test]
    fn missing_or_invalid_collection_counts_are_none() {
        assert_eq!(collection_total(None), None);
        assert_eq!(
            collection_total(Some(&json!("https://example.com/likes"))),
            None
        );
        assert_eq!(
            collection_total(Some(&json!({ "totalItems": -3 }))),
            Some(0)
        );
    }

    #[test]
    fn like_and_announce_targets_are_normalized() {
        let payload = json!({
            "id": "https://mastodon.social/users/ana#likes/1",
            "type": "Like",
            "object": "https://moang.com.br/users/caio/posts/abc/"
        });

        assert_eq!(
            reaction_ids(&payload),
            Some((
                "https://mastodon.social/users/ana#likes/1".to_string(),
                "https://moang.com.br/users/caio/posts/abc".to_string()
            ))
        );
    }

    #[test]
    fn reactions_can_embed_the_target_object() {
        let payload = json!({
            "id": "https://example.com/activities/announce/1",
            "type": "Announce",
            "object": { "id": "https://moang.com.br/users/caio/posts/abc" }
        });

        assert_eq!(
            reaction_ids(&payload),
            Some((
                "https://example.com/activities/announce/1".to_string(),
                "https://moang.com.br/users/caio/posts/abc".to_string()
            ))
        );
    }

    #[test]
    fn undo_targets_can_be_embedded_or_referenced_by_id() {
        assert_eq!(
            undo_target_id(&json!({ "object": "https://example.com/likes/1" })),
            Some("https://example.com/likes/1".to_string())
        );
        assert_eq!(
            undo_target_id(&json!({
                "object": { "id": "https://example.com/likes/2", "type": "Like" }
            })),
            Some("https://example.com/likes/2".to_string())
        );
        assert_eq!(undo_target_id(&json!({ "type": "Undo" })), None);
    }

    #[test]
    fn audience_accepts_arrays_strings_and_embedded_objects() {
        assert_eq!(
            ids_of(Some(&json!([
                "https://www.w3.org/ns/activitystreams#Public",
                { "id": "https://example.com/users/ana" },
                42
            ]))),
            vec![
                "https://www.w3.org/ns/activitystreams#Public".to_string(),
                "https://example.com/users/ana".to_string()
            ]
        );
        assert_eq!(
            ids_of(Some(&json!("https://example.com/users/ana"))),
            vec!["https://example.com/users/ana".to_string()]
        );
        assert!(ids_of(None).is_empty());
    }

    #[test]
    fn ids_are_read_from_strings_or_objects() {
        assert_eq!(
            id_of(&json!("https://example.com/notes/1")),
            Some("https://example.com/notes/1".to_string())
        );
        assert_eq!(
            id_of(&json!({ "id": "https://example.com/notes/2" })),
            Some("https://example.com/notes/2".to_string())
        );
        assert_eq!(id_of(&json!({ "type": "Note" })), None);
    }

    #[test]
    fn only_rfc3339_timestamps_are_accepted() {
        assert!(parse_time("2026-11-08T01:02:03Z").is_some());
        assert!(parse_time("08/11/2026").is_none());
    }

    #[test]
    fn urls_accept_strings_objects_and_arrays() {
        assert_eq!(
            first_url(&json!("https://example.com/1")),
            Some("https://example.com/1".to_string())
        );
        assert_eq!(
            first_url(&json!({ "href": "https://example.com/2", "type": "Link" })),
            Some("https://example.com/2".to_string())
        );
        assert_eq!(
            first_url(&json!([
                { "href": "https://example.com/3" },
                "https://example.com/4"
            ])),
            Some("https://example.com/3".to_string())
        );
        assert_eq!(first_url(&json!(42)), None);
    }
}
