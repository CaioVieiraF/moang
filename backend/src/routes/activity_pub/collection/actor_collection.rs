use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::{ActivityPub, ObjType, Url};

/// A plain `OrderedCollection` of actor ids.
///
/// Followers and following are lists of actors, not lists of outbox activities,
/// so they have their own small type instead of reusing `OrderedItem`.
#[derive(Deserialize, Serialize)]
pub struct ActorCollection {
    #[serde(flatten)]
    activity_pub: ActivityPub,

    #[serde(rename = "totalItems")]
    total_items: i64,

    #[serde(rename = "orderedItems")]
    ordered_items: Vec<Url>,
}

impl ActorCollection {
    pub fn new(id: &str, actors: Vec<String>) -> Result<ActorCollection, String> {
        let ordered_items = actors
            .into_iter()
            .map(Url::try_from)
            .collect::<Result<Vec<Url>, String>>()?;

        Ok(ActorCollection {
            activity_pub: ActivityPub::new(Url::try_from(id)?, ObjType::OrderedCollection),
            total_items: ordered_items.len() as i64,
            ordered_items,
        })
    }
}
