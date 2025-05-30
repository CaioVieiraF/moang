use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::{ObjType, Url};

use super::OrderedItem;

#[derive(Deserialize, Serialize)]
pub struct OrderedCollection {
    id: Url,

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "totalItems")]
    total_items: i64,

    #[serde(rename = "orderedItems")]
    ordered_items: Option<Vec<OrderedItem>>,
}

#[derive(Default)]
pub struct NoId;
#[derive(Default)]
pub struct NoTotalItems;

#[derive(Default)]
pub struct OrderedCollectionBuilder<I, T> {
    id: I,
    total_items: T,
    ordered_items: Option<Vec<OrderedItem>>,
}

impl OrderedCollectionBuilder<NoId, NoTotalItems> {
    pub fn new() -> Self {
        OrderedCollectionBuilder {
            ..Default::default()
        }
    }
}

impl<I> OrderedCollectionBuilder<I, NoTotalItems> {
    pub fn id(self, value: impl Into<String>) -> OrderedCollectionBuilder<Url, NoTotalItems> {
        OrderedCollectionBuilder {
            id: Url::try_from(value.into()).unwrap(),
            total_items: self.total_items,
            ordered_items: self.ordered_items,
        }
    }
}

impl<T> OrderedCollectionBuilder<Url, T> {
    pub fn total_items(self, value: i64) -> OrderedCollectionBuilder<Url, i64> {
        OrderedCollectionBuilder {
            id: self.id,
            total_items: value,
            ordered_items: self.ordered_items,
        }
    }
}

impl<I, T> OrderedCollectionBuilder<I, T> {
    pub fn ordered_items(self, value: Vec<OrderedItem>) -> OrderedCollectionBuilder<I, T> {
        OrderedCollectionBuilder {
            id: self.id,
            total_items: self.total_items,
            ordered_items: Some(value),
        }
    }
}

impl OrderedCollectionBuilder<Url, i64> {
    pub fn build(self) -> OrderedCollection {
        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();

        OrderedCollection {
            context,
            id: self.id,
            obj_type: ObjType::OrderedCollection,
            total_items: self.total_items,
            ordered_items: self.ordered_items,
        }
    }
}
