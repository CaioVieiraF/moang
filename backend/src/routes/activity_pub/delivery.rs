use std::{
    collections::HashSet,
    time::{Duration, SystemTime},
};

use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    establish_connection,
    models::{Activity, Delivery, NewDelivery, Post, User},
    schema::{activities, deliveries, followers, posts, remote_actors, users},
};

use super::{
    collection::OrderedItem,
    fetch::validate_remote_url,
    keys::get_or_create_key,
    signature::sign_request,
};

pub fn save_activity(
    id: &str,
    user_id: Option<&str>,
    actor_id: &str,
    activity_type: &str,
    object_id: Option<&str>,
    direction: &str,
    payload: Value,
) -> Result<(), String> {
    let connection = &mut establish_connection();
    let activity = Activity {
        id: id.to_string(),
        user_id: user_id.map(str::to_string),
        actor_id: actor_id.to_string(),
        activity_type: activity_type.to_string(),
        object_id: object_id.map(str::to_string),
        direction: direction.to_string(),
        payload,
        created_at: SystemTime::now(),
    };

    diesel::insert_into(activities::table)
        .values(&activity)
        .on_conflict_do_nothing()
        .execute(connection)
        .map_err(|error| error.to_string())?;

    Ok(())
}

/// Queue one activity for every follower of a local user.
///
/// Shared inboxes are used when available, which keeps the number of HTTP
/// requests small.
pub fn enqueue_for_followers(user_id: &str, activity_id: &str) -> Result<(), String> {
    let connection = &mut establish_connection();
    let inboxes = followers::table
        .inner_join(remote_actors::table)
        .filter(followers::user_id.eq(user_id))
        .filter(followers::accepted.eq(true))
        .select((
            remote_actors::inbox_url,
            remote_actors::shared_inbox_url,
        ))
        .load::<(String, Option<String>)>(connection)
        .map_err(|error| error.to_string())?;

    let unique: HashSet<String> = inboxes
        .into_iter()
        .map(|(inbox, shared)| shared.unwrap_or(inbox))
        .collect();

    enqueue_activity(activity_id, &unique.into_iter().collect::<Vec<_>>())
}

pub fn enqueue_activity(activity_id: &str, inboxes: &[String]) -> Result<(), String> {
    if inboxes.is_empty() {
        return Ok(());
    }

    let connection = &mut establish_connection();
    let now = SystemTime::now();
    let rows = inboxes
        .iter()
        .map(|inbox| NewDelivery {
            activity_id: activity_id.to_string(),
            inbox_url: inbox.clone(),
            state: "pending".to_string(),
            attempts: 0,
            next_attempt_at: now,
        })
        .collect::<Vec<_>>();

    diesel::insert_into(deliveries::table)
        .values(&rows)
        .on_conflict_do_nothing()
        .execute(connection)
        .map_err(|error| error.to_string())?;

    Ok(())
}

/// Send the user's existing posts to a newly accepted follower.
///
/// Mastodon uses the outbox `totalItems` as the profile post count, but it does
/// not import the outbox itself: only `Create` activities delivered to the
/// follower become statuses. Without this backfill a new follower sees the
/// post count and no posts, because everything was written before the follow.
pub fn backfill_for_follower(user: &User, inbox_url: &str) -> Result<usize, String> {
    let connection = &mut establish_connection();
    let existing_posts = posts::table
        .filter(posts::author.eq(&user.id))
        .filter(posts::visibility.ne("private"))
        .order(posts::created_at.asc())
        .select(Post::as_select())
        .load::<Post>(connection)
        .map_err(|error| error.to_string())?;

    let actor = AppConfig::get().actor_url(&user.name);
    let inbox = [inbox_url.to_string()];
    let mut queued = 0;

    for post in &existing_posts {
        let activity = OrderedItem::from_post(post, &user.name)?;
        let payload = serde_json::to_value(&activity).map_err(|error| error.to_string())?;
        let activity_id = payload
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| "backfill activity is missing id".to_string())?
            .to_string();
        let object_id = payload
            .get("object")
            .and_then(|object| object.get("id"))
            .and_then(Value::as_str)
            .map(str::to_string);

        save_activity(
            &activity_id,
            Some(&user.id),
            &actor,
            "Create",
            object_id.as_deref(),
            "out",
            payload.clone(),
        )?;

        // The stored activity may predate the current post (for example after an
        // edit), so refresh the payload before delivering it again.
        diesel::update(activities::table.find(&activity_id))
            .set(activities::payload.eq(&payload))
            .execute(connection)
            .map_err(|error| error.to_string())?;

        enqueue_activity(&activity_id, &inbox)?;
        queued += 1;
    }

    Ok(queued)
}

/// Background worker that delivers queued activities.
///
/// Delivery is intentionally persistent: a restart does not lose pending work.
pub fn spawn_worker() {
    tokio::spawn(async move {
        let client = match reqwest::Client::builder()
            .user_agent(AppConfig::get().user_agent())
            .timeout(Duration::from_secs(15))
            .redirect(reqwest::redirect::Policy::none())
            .build()
        {
            Ok(client) => client,
            Err(error) => {
                log::error!("could not create delivery client: {error}");
                return;
            }
        };

        loop {
            match process_batch(&client).await {
                Ok(count) if count > 0 => log::info!("delivered {count} activities"),
                Ok(_) => {}
                Err(error) => log::error!("delivery worker error: {error}"),
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

async fn process_batch(client: &reqwest::Client) -> Result<usize, String> {
    let pending = {
        let connection = &mut establish_connection();
        deliveries::table
            .filter(deliveries::state.eq("pending"))
            .filter(deliveries::next_attempt_at.le(SystemTime::now()))
            .order(deliveries::id.asc())
            .limit(20)
            .select(Delivery::as_select())
            .load::<Delivery>(connection)
            .map_err(|error| error.to_string())?
    };

    let mut delivered = 0;
    for delivery in pending {
        if deliver_one(client, &delivery).await.is_ok() {
            delivered += 1;
        }
    }

    Ok(delivered)
}

async fn deliver_one(client: &reqwest::Client, delivery: &Delivery) -> Result<(), String> {
    let (activity, user) = {
        let connection = &mut establish_connection();
        let activity = activities::table
            .find(&delivery.activity_id)
            .select(Activity::as_select())
            .first::<Activity>(connection)
            .optional()
            .map_err(|error| error.to_string())?
            .ok_or_else(|| format!("activity {} not found", delivery.activity_id))?;

        let user_id = activity
            .user_id
            .clone()
            .ok_or_else(|| "outbound activity has no local user".to_string())?;
        let user = users::table
            .find(user_id)
            .select(User::as_select())
            .first::<User>(connection)
            .map_err(|error| error.to_string())?;

        (activity, user)
    };

    let result = send_activity(client, delivery, &activity, &user).await;
    record_result(delivery, result.as_ref().err().map(String::as_str));

    result
}

async fn send_activity(
    client: &reqwest::Client,
    delivery: &Delivery,
    activity: &Activity,
    user: &User,
) -> Result<(), String> {
    let config = AppConfig::get();
    let inbox = validate_remote_url(&delivery.inbox_url)?;
    let body = serde_json::to_vec(&activity.payload).map_err(|error| error.to_string())?;
    let key = get_or_create_key(&user.id)?;
    let key_id = config.actor_key_id(&user.name);
    let headers = sign_request("POST", &inbox, &body, &key, &key_id)?;

    let response = client
        .post(inbox)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if response.status().is_success() {
        return Ok(());
    }

    Err(format!("HTTP {}", response.status()))
}

fn record_result(delivery: &Delivery, error: Option<&str>) {
    let connection = &mut establish_connection();
    let now = SystemTime::now();
    let attempts = delivery.attempts + 1;
    let max_attempts = AppConfig::get().max_delivery_attempts();

    let state = match error {
        None => "done",
        Some(_) if attempts >= max_attempts => "failed",
        Some(_) => "pending",
    };

    let next_attempt_at = if state == "pending" {
        now + Duration::from_secs((2_u64.pow(attempts.min(10) as u32)).min(3600))
    } else {
        now
    };

    let _ = diesel::update(deliveries::table.find(delivery.id))
        .set((
            deliveries::state.eq(state),
            deliveries::attempts.eq(attempts),
            deliveries::next_attempt_at.eq(next_attempt_at),
            deliveries::last_error.eq(error),
            deliveries::updated_at.eq(now),
        ))
        .execute(connection);
}

/// Create a unique activity id for updates, deletes and other one-off activities.
pub fn new_activity_id(username: &str) -> (String, Uuid) {
    let id = Uuid::new_v4();
    (
        AppConfig::get().activity_url(username, &id),
        id,
    )
}
