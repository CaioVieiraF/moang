use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    models::{ActorKey, User},
};

use super::{ActivityPub, Context, ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Actor {
    #[serde(flatten)]
    activity_pub: ActivityPub,

    inbox: Url,
    outbox: Url,
    following: Url,
    followers: Url,
    url: Url,
    published: String,
    summary: String,
    discoverable: bool,
    indexable: bool,
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,

    #[serde(rename = "preferredUsername")]
    preferred_username: String,

    #[serde(rename = "manuallyApprovesFollowers")]
    manually_approves_followers: bool,

    #[serde(rename = "publicKey")]
    public_key: PubKey,

    endpoints: Endpoints,
}

#[derive(Deserialize, Serialize)]
struct Icon {
    #[serde(rename = "type")]
    obj_type: ObjType,
    url: Url,

    #[serde(rename = "mediaType")]
    media_type: String,
}

impl Icon {
    fn new(url: impl Into<String>) -> Result<Icon, String> {
        Ok(Icon {
            obj_type: ObjType::Image,
            url: Url::try_from(url.into())?,
            media_type: "image/png".into(),
        })
    }
}

#[derive(Deserialize, Serialize)]
struct PubKey {
    id: Url,
    owner: Url,
    #[serde(rename = "publicKeyPem")]
    public_key_pem: String,
}

#[derive(Deserialize, Serialize)]
struct Endpoints {
    #[serde(rename = "sharedInbox")]
    shared_inbox: Url,
}

impl Actor {
    pub fn from_user(user: &User, key: &ActorKey) -> Result<Actor, String> {
        let config = AppConfig::get();
        let actor_url = config.actor_url(&user.name);
        let actor_url = Url::try_from(actor_url)?;

        let activity_pub = ActivityPub::with_context(
            actor_url.clone(),
            ObjType::Person,
            Context::with_security(),
        );

        let published: DateTime<Utc> = user.created_at.into();
        let display_name = if user.display_name.trim().is_empty() {
            user.name.clone()
        } else {
            user.display_name.clone()
        };

        let icon = if user.avatar_url.trim().is_empty() {
            None
        } else {
            Some(Icon::new(user.avatar_url.clone())?)
        };

        Ok(Actor {
            activity_pub,
            inbox: Url::try_from(config.inbox_url(&user.name))?,
            outbox: Url::try_from(config.outbox_url(&user.name))?,
            following: Url::try_from(config.following_url(&user.name))?,
            followers: Url::try_from(config.followers_url(&user.name))?,
            url: Url::try_from(config.profile_url())?,
            published: format!("{}", published.format("%+")),
            summary: user.summary.clone(),
            discoverable: user.is_discoverable,
            indexable: true,
            name: display_name,
            icon,
            preferred_username: user.name.clone(),
            manually_approves_followers: user.is_locked,
            public_key: PubKey {
                id: Url::try_from(config.actor_key_id(&user.name))?,
                owner: actor_url,
                public_key_pem: key.public_key_pem.clone(),
            },
            endpoints: Endpoints {
                shared_inbox: Url::try_from(config.shared_inbox_url())?,
            },
        })
    }
}
