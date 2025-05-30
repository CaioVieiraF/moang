use std::str::FromStr;

use actix_web::http::{uri::InvalidUri, Uri};
use serde::{Deserialize, Serialize};

pub mod actor;
pub mod collection;
pub mod content_object;

#[derive(Deserialize, Serialize, Clone)]
pub struct Url(String);

#[derive(Deserialize, Serialize, Clone)]
pub struct ActivityPub {
    #[serde(rename = "@context")]
    context: Url,
    #[serde(rename = "type")]
    obj_type: ObjType,
    id: Url,
}

impl ActivityPub {
    pub fn new(entity_id: Url, obj_type: ObjType) -> ActivityPub {
        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();
        ActivityPub {
            context,
            obj_type,
            id: entity_id,
        }
    }
}

impl TryFrom<String> for Url {
    type Error = InvalidUri;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _ = Uri::from_str(&value)?;
        Ok(Url(value))
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ObjType {
    Note,
    Person,
    Create,
    Article,
    Collection,
    OrderedCollection,
    OrderedCollectionPage,
}
