use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::json;

use crate::{
    config::AppConfig,
    establish_connection,
    models::{Post, User},
    routes::activity_pub::{
        actor::Actor,
        collection::OrderedItem,
        content_object::ContentObject,
        delivery::{enqueue_for_followers, new_activity_id, save_activity},
        keys::get_or_create_key,
        Context, ObjType, Url, PUBLIC_COLLECTION,
    },
    schema::users,
};

pub fn publish_post(post: &Post, author_id: &str) {
    if let Err(error) = try_publish_post(post, author_id) {
        log::error!("could not publish post {}: {error}", post.id);
    }
}

pub fn publish_update(post: &Post, author_id: &str) {
    if let Err(error) = try_publish_update(post, author_id) {
        log::error!("could not publish update for post {}: {error}", post.id);
    }
}

pub fn publish_delete(post: &Post, author_id: &str) {
    if let Err(error) = try_publish_delete(post, author_id) {
        log::error!("could not publish delete for post {}: {error}", post.id);
    }
}

/// Tell followers that the actor changed (display name, bio, avatar, banner).
pub fn publish_actor_update(user: &User) {
    if let Err(error) = try_publish_actor_update(user) {
        log::error!("could not publish actor update for @{}: {error}", user.name);
    }
}

fn try_publish_post(post: &Post, author_id: &str) -> Result<(), String> {
    if post.visibility == "private" {
        return Ok(());
    }

    let user = local_user(author_id)?;
    let activity = OrderedItem::from_post(post, &user.name)?;
    let payload = serde_json::to_value(&activity).map_err(|error| error.to_string())?;
    save_and_enqueue(&payload, author_id, &user.name)?;
    Ok(())
}

fn try_publish_update(post: &Post, author_id: &str) -> Result<(), String> {
    if post.visibility == "private" {
        return Ok(());
    }

    let user = local_user(author_id)?;
    let object = ContentObject::from_post(post, &user.name)?;
    let object_value = serde_json::to_value(&object).map_err(|error| error.to_string())?;
    let (activity_id, _) = new_activity_id(&user.name);
    let published: DateTime<Utc> = post.updated_at.into();
    let to = object.to().to_vec();
    let cc = object.cc().to_vec();

    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": ObjType::Update,
        "actor": Url::try_from(crate::config::AppConfig::get().actor_url(&user.name))?,
        "published": format!("{}", published.format("%+")),
        "to": to,
        "cc": cc,
        "object": object_value
    });

    save_and_enqueue(&payload, author_id, &user.name)?;
    Ok(())
}

fn try_publish_delete(post: &Post, author_id: &str) -> Result<(), String> {
    let user = local_user(author_id)?;
    let object_id = crate::config::AppConfig::get().object_url(&user.name, &post.ap_id);
    let (activity_id, _) = new_activity_id(&user.name);
    let deleted: DateTime<Utc> = Utc::now();

    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": ObjType::Delete,
        "actor": Url::try_from(crate::config::AppConfig::get().actor_url(&user.name))?,
        "published": format!("{}", deleted.format("%+")),
        "to": [Url::try_from(PUBLIC_COLLECTION)?],
        "object": {
            "id": object_id,
            "type": ObjType::Tombstone,
            "formerType": ObjType::Article,
            "deleted": format!("{}", deleted.format("%+"))
        }
    });

    save_and_enqueue(&payload, author_id, &user.name)?;
    Ok(())
}

fn try_publish_actor_update(user: &User) -> Result<(), String> {
    let config = AppConfig::get();
    let key = get_or_create_key(&user.id)?;
    let actor = Actor::from_user(user, &key)?;
    let (activity_id, _) = new_activity_id(&user.name);

    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": ObjType::Update,
        "actor": Url::try_from(config.actor_url(&user.name))?,
        "to": [Url::try_from(PUBLIC_COLLECTION)?],
        "cc": [Url::try_from(config.followers_url(&user.name))?],
        "object": serde_json::to_value(&actor).map_err(|error| error.to_string())?,
    });

    save_and_enqueue(&payload, &user.id, &user.name)
}

fn save_and_enqueue(
    payload: &serde_json::Value,
    author_id: &str,
    username: &str,
) -> Result<(), String> {
    let activity_id = payload
        .get("id")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "activity is missing id".to_string())?;
    let actor_id = payload
        .get("actor")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "activity is missing actor".to_string())?;
    let activity_type = payload
        .get("type")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "activity is missing type".to_string())?;
    let object_id = payload
        .get("object")
        .and_then(|object| object.get("id"))
        .and_then(serde_json::Value::as_str);

    save_activity(
        activity_id,
        Some(author_id),
        actor_id,
        activity_type,
        object_id,
        "out",
        payload.clone(),
    )?;
    enqueue_for_followers(author_id, activity_id)?;

    log::info!("queued {activity_type} for @{username}");
    Ok(())
}

fn local_user(author_id: &str) -> Result<User, String> {
    let connection = &mut establish_connection();
    users::table
        .find(author_id)
        .select(User::as_select())
        .first::<User>(connection)
        .map_err(|error| error.to_string())
}
