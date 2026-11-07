use std::{
    net::IpAddr,
    time::{Duration, SystemTime},
};

use diesel::prelude::*;
use reqwest::header::ACCEPT;
use serde_json::Value;
use url::Url;

use crate::{config::AppConfig, establish_connection, models::RemoteActor, schema::remote_actors};

const ACTIVITY_JSON: &str = "application/activity+json";
const ACTIVITY_LD_JSON: &str =
    "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";

/// Fetch a remote actor and cache it locally.
///
/// `force` is used when a signature points at a key we do not know yet, which is
/// how key rotation is picked up.
pub async fn fetch_remote_actor(actor_url: &str, force: bool) -> Result<RemoteActor, String> {
    let config = AppConfig::get();
    let url = validate_remote_url(actor_url)?;

    if !force {
        if let Some(cached) = cached_actor(actor_url)? {
            let age = SystemTime::now()
                .duration_since(cached.fetched_at)
                .unwrap_or_default();
            if age < Duration::from_secs(3600) {
                return Ok(cached);
            }
        }
    }

    let client = reqwest::Client::builder()
        .user_agent(config.user_agent())
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()
        .map_err(|error| error.to_string())?;

    let response = client
        .get(url.clone())
        .header(ACCEPT, format!("{ACTIVITY_JSON}, {ACTIVITY_LD_JSON}"))
        .send()
        .await
        .map_err(|error| format!("error fetching {actor_url}: {error}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "error fetching {actor_url}: HTTP {}",
            response.status()
        ));
    }

    let body: Value = response
        .json()
        .await
        .map_err(|error| format!("invalid actor JSON from {actor_url}: {error}"))?;

    let actor = parse_actor(&body, url.host_str().unwrap_or_default())?;
    store_actor(&actor)?;
    Ok(actor)
}

pub fn cached_actor(actor_id: &str) -> Result<Option<RemoteActor>, String> {
    let connection = &mut establish_connection();
    remote_actors::table
        .find(actor_id)
        .select(RemoteActor::as_select())
        .first(connection)
        .optional()
        .map_err(|error| error.to_string())
}

fn parse_actor(value: &Value, expected_host: &str) -> Result<RemoteActor, String> {
    let id = value
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| "remote actor is missing id".to_string())?;

    let parsed_id = validate_remote_url(id)?;
    if parsed_id.host_str() != Some(expected_host) {
        return Err(format!("actor id host does not match requested host: {id}"));
    }

    let preferred_username = value
        .get("preferredUsername")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let inbox_url = value
        .get("inbox")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("remote actor {id} is missing inbox"))?
        .to_string();

    let shared_inbox_url = value
        .get("endpoints")
        .and_then(|endpoints| endpoints.get("sharedInbox"))
        .and_then(Value::as_str)
        .map(str::to_string);

    let public_key = value
        .get("publicKey")
        .ok_or_else(|| format!("remote actor {id} is missing publicKey"))?;

    let public_key_id = public_key
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("remote actor {id} is missing publicKey.id"))?
        .to_string();

    let public_key_pem = public_key
        .get("publicKeyPem")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("remote actor {id} is missing publicKeyPem"))?
        .to_string();

    let name = value
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(&preferred_username)
        .to_string();

    let summary = value
        .get("summary")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let avatar_url = value
        .get("icon")
        .and_then(|icon| {
            icon.as_str()
                .map(str::to_string)
                .or_else(|| icon.get("url").and_then(Value::as_str).map(str::to_string))
        })
        .unwrap_or_default();

    Ok(RemoteActor {
        id: id.to_string(),
        preferred_username,
        domain: parsed_id.host_str().unwrap_or_default().to_string(),
        name,
        summary,
        avatar_url,
        inbox_url,
        shared_inbox_url,
        public_key_id,
        public_key_pem,
        fetched_at: SystemTime::now(),
    })
}

fn store_actor(actor: &RemoteActor) -> Result<(), String> {
    let connection = &mut establish_connection();
    let exists = remote_actors::table
        .find(&actor.id)
        .select(remote_actors::id)
        .first::<String>(connection)
        .optional()
        .map_err(|error| error.to_string())?
        .is_some();

    if exists {
        diesel::update(remote_actors::table.find(&actor.id))
            .set((
                remote_actors::preferred_username.eq(&actor.preferred_username),
                remote_actors::domain.eq(&actor.domain),
                remote_actors::name.eq(&actor.name),
                remote_actors::summary.eq(&actor.summary),
                remote_actors::avatar_url.eq(&actor.avatar_url),
                remote_actors::inbox_url.eq(&actor.inbox_url),
                remote_actors::shared_inbox_url.eq(&actor.shared_inbox_url),
                remote_actors::public_key_id.eq(&actor.public_key_id),
                remote_actors::public_key_pem.eq(&actor.public_key_pem),
                remote_actors::fetched_at.eq(actor.fetched_at),
            ))
            .execute(connection)
            .map_err(|error| error.to_string())?;
    } else {
        diesel::insert_into(remote_actors::table)
            .values(actor)
            .execute(connection)
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

/// Reject URLs that could make the server talk to itself or to internal services.
pub fn validate_remote_url(input: &str) -> Result<Url, String> {
    let url = Url::parse(input).map_err(|error| error.to_string())?;

    if url.scheme() != "https" {
        return Err(format!("remote URL must use https: {input}"));
    }

    if url.username() != "" || url.password().is_some() {
        return Err(format!("remote URL must not contain credentials: {input}"));
    }

    let host = url
        .host_str()
        .ok_or_else(|| format!("remote URL is missing a host: {input}"))?;

    if host.eq_ignore_ascii_case("localhost") {
        return Err(format!("remote URL points to localhost: {input}"));
    }

    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_private_ip(ip) {
            return Err(format!("remote URL points to a private address: {input}"));
        }
    }

    Ok(url)
}

fn is_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            ip.is_loopback()
                || ip.is_private()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_unspecified()
                || ip.octets()[0] == 0
                || (ip.octets()[0] == 100 && (64..=127).contains(&ip.octets()[1]))
        }
        IpAddr::V6(ip) => {
            ip.is_loopback()
                || ip.is_unspecified()
                || ip.is_unique_local()
                || ip.is_unicast_link_local()
        }
    }
}
