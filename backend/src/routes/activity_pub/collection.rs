use serde::{Deserialize, Serialize};

use super::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct OrderedCollection {
    id: Url,
    first: Url,

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "totalItems")]
    total_items: i32,
}

pub struct NoId;
pub struct NoFirst;
pub struct NoTotalItems;

pub struct OrderedCollectionBuilder<I, F, T> {
    id: I,
    first: F,
    total_items: T,
}

impl OrderedCollectionBuilder<NoId, NoFirst, NoTotalItems> {
    pub fn new() -> Self {
        OrderedCollectionBuilder {
            id: NoId,
            first: NoFirst,
            total_items: NoTotalItems,
        }
    }
}

impl<I, F, T> OrderedCollectionBuilder<I, F, T> {
    pub fn id(self, value: String) -> OrderedCollectionBuilder<Url, F, T> {
        OrderedCollectionBuilder {
            id: Url::try_from(value).unwrap(),
            first: self.first,
            total_items: self.total_items,
        }
    }

    pub fn first(self, value: String) -> OrderedCollectionBuilder<I, Url, T> {
        OrderedCollectionBuilder {
            id: self.id,
            first: Url::try_from(value).unwrap(),
            total_items: self.total_items,
        }
    }

    pub fn total_items(self, value: i32) -> OrderedCollectionBuilder<I, F, i32> {
        OrderedCollectionBuilder {
            id: self.id,
            first: self.first,
            total_items: value,
        }
    }
}

impl OrderedCollectionBuilder<Url, Url, i32> {
    pub fn build(self) -> OrderedCollection {
        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();

        OrderedCollection {
            context,
            id: self.id,
            first: self.first,
            obj_type: ObjType::OrderedCollection,
            total_items: self.total_items,
        }
    }
}
