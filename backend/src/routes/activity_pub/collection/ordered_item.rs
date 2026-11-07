use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    models::Post,
};

use super::super::{content_object::ContentObject, Context, ObjType, Url};

/// A `Create` activity, used by the outbox.
///
/// The activity id is the old `/outbox/posts/{id}` URL so the existing frontend
/// can keep reading posts by their numeric id. The object id is the canonical
/// ActivityPub object URL (`/users/{name}/posts/{uuid}`).
#[derive(Deserialize, Serialize)]
pub struct OrderedItem {
    #[serde(rename = "@context")]
    context: Context,

    id: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    actor: Url,
    published: String,
    to: Vec<Url>,
    cc: Vec<Url>,
    object: ContentObject,
    name: String,
}

impl OrderedItem {
    pub fn from_post(value: &Post, username: &str) -> Result<OrderedItem, String> {
        let config = AppConfig::get();
        let object = ContentObject::from_post(value, username)?;
        let date: DateTime<Utc> = value.created_at.into();

        Ok(OrderedItem {
            context: Context::activity_streams(),
            id: Url::try_from(format!(
                "{}/outbox/posts/{}",
                config.actor_url(username),
                value.id
            ))?,
            obj_type: ObjType::Create,
            actor: Url::try_from(config.actor_url(username))?,
            published: format!("{}", date.format("%+")),
            to: object.to().to_vec(),
            cc: object.cc().to_vec(),
            object,
            name: value.title.clone(),
        })
    }
}
