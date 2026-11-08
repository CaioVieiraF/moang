use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Collection {
    id: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "totalItems")]
    total_items: i32,
}

impl Collection {
    pub fn new(id: Url) -> Collection {
        Self::with_total(id, 0)
    }

    /// The endpoints behind these ids all answer with `OrderedCollection`, so
    /// the stub must advertise the same type. Consumers that look for `items`
    /// on a `Collection` would otherwise miss the `orderedItems` page.
    pub fn with_total(id: Url, total_items: i32) -> Collection {
        Collection {
            id,
            obj_type: ObjType::OrderedCollection,
            total_items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Collection;
    use crate::routes::activity_pub::Url;

    #[test]
    fn count_stubs_match_the_ordered_collection_endpoints() {
        let collection = Collection::with_total(
            Url::try_from("https://example.com/posts/1/replies").unwrap(),
            3,
        );
        let value = serde_json::to_value(&collection).unwrap();

        assert_eq!(value["type"], "OrderedCollection");
        assert_eq!(value["totalItems"], 3);
    }
}
