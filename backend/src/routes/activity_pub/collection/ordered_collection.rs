use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::routes::activity_pub::{ActivityPub, ObjType, Url};

use super::OrderedItem;

#[derive(Deserialize, Serialize)]
pub struct OrderedCollection {
    #[serde(flatten)]
    activity_pub: ActivityPub,

    #[serde(rename = "totalItems")]
    total_items: i64,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
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
    first: Option<Url>,
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
            first: self.first,
        }
    }
}

impl<T> OrderedCollectionBuilder<Url, T> {
    pub fn total_items(self, value: i64) -> OrderedCollectionBuilder<Url, i64> {
        OrderedCollectionBuilder {
            id: self.id,
            total_items: value,
            ordered_items: self.ordered_items,
            first: self.first,
        }
    }
}

impl<T> OrderedCollectionBuilder<Url, T> {
    pub fn ordered_items(self, value: Vec<OrderedItem>) -> OrderedCollectionBuilder<Url, T> {
        OrderedCollectionBuilder {
            ordered_items: Some(value),
            ..self
        }
    }

    pub fn first(self, value: impl Display) -> OrderedCollectionBuilder<Url, T> {
        let first = format!("{}/{value}", self.id.0);
        OrderedCollectionBuilder {
            first: Some(first.try_into().unwrap()),
            ..self
        }
    }
}

impl OrderedCollectionBuilder<Url, i64> {
    pub fn build(self) -> OrderedCollection {
        let base = ActivityPub::new(self.id, ObjType::OrderedCollection);
        let mut extra = HashMap::new();

        if let Some(ordered_items) = self.ordered_items {
            extra.insert("ordered_items".to_string(), json!(ordered_items));
        }

        if let Some(first) = self.first {
            extra.insert("first".to_string(), json!(first));
        }

        OrderedCollection {
            activity_pub: base,
            total_items: self.total_items,
            extra,
        }
    }
}
