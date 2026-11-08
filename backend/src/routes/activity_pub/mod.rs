use std::fmt::Display;

use serde::{Deserialize, Serialize};
use url::Url as ParsedUrl;

pub mod actor;
pub mod collection;
pub mod content_object;
pub mod delivery;
pub mod fetch;
pub mod inbox;
pub mod keys;
pub mod remote_note;
pub mod signature;

pub const ACTIVITY_STREAMS_CONTEXT: &str = "https://www.w3.org/ns/activitystreams";
pub const SECURITY_CONTEXT: &str = "https://w3id.org/security/v1";
pub const PUBLIC_COLLECTION: &str = "https://www.w3.org/ns/activitystreams#Public";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Url(String);

impl Url {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Url {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = ParsedUrl::parse(&value).map_err(|error| error.to_string())?;
        if parsed.scheme() != "http" && parsed.scheme() != "https" {
            return Err(format!("unsupported URL scheme in {value}"));
        }
        Ok(Url(value))
    }
}

impl TryFrom<&str> for Url {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Url::try_from(value.to_string())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Context {
    One(String),
    Many(Vec<String>),
}

impl Context {
    pub fn activity_streams() -> Self {
        Context::One(ACTIVITY_STREAMS_CONTEXT.to_string())
    }

    pub fn with_security() -> Self {
        Context::Many(vec![
            ACTIVITY_STREAMS_CONTEXT.to_string(),
            SECURITY_CONTEXT.to_string(),
        ])
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActivityPub {
    #[serde(rename = "@context")]
    pub context: Context,

    #[serde(rename = "type")]
    pub obj_type: ObjType,

    pub id: Url,
}

impl ActivityPub {
    pub fn new(entity_id: Url, obj_type: ObjType) -> ActivityPub {
        ActivityPub {
            context: Context::activity_streams(),
            obj_type,
            id: entity_id,
        }
    }

    pub fn with_context(entity_id: Url, obj_type: ObjType, context: Context) -> ActivityPub {
        ActivityPub {
            context,
            obj_type,
            id: entity_id,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjType {
    Note,
    Article,
    Person,
    Service,
    Create,
    Update,
    Delete,
    Follow,
    Accept,
    Reject,
    Undo,
    Announce,
    Like,
    Block,
    Collection,
    OrderedCollection,
    OrderedCollectionPage,
    Tombstone,
    Image,
    Document,
    Hashtag,
    Mention,
    PropertyValue,
    Page,
    Video,
    Question,
}
