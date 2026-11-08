use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::{ActivityPub, ObjType, Url};

use super::OrderedItem;

#[derive(Deserialize, Serialize)]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    activity_pub: ActivityPub,

    #[serde(rename = "totalItems")]
    total_items: i64,

    #[serde(rename = "partOf")]
    part_of: Url,

    #[serde(rename = "orderedItems")]
    ordered_items: Vec<OrderedItem>,
}

pub struct NoId;
pub struct NoTotalItems;
pub struct NoOrderedItems;

pub struct OrderedCollectionPageBuilder<I, T, O> {
    id: I,
    total_items: T,
    ordered_items: O,
    part_of: Option<Url>,
}

impl OrderedCollectionPageBuilder<NoId, NoTotalItems, NoOrderedItems> {
    pub fn new() -> Self {
        OrderedCollectionPageBuilder {
            id: NoId,
            total_items: NoTotalItems,
            ordered_items: NoOrderedItems,
            part_of: None,
        }
    }
}

impl<I> OrderedCollectionPageBuilder<I, NoTotalItems, NoOrderedItems> {
    pub fn id(
        self,
        value: impl Into<String>,
    ) -> OrderedCollectionPageBuilder<Url, NoTotalItems, NoOrderedItems> {
        OrderedCollectionPageBuilder {
            id: Url::try_from(value.into()).unwrap(),
            total_items: self.total_items,
            ordered_items: self.ordered_items,
            part_of: self.part_of,
        }
    }
}

impl<T, O> OrderedCollectionPageBuilder<Url, T, O> {
    pub fn total_items(self, value: i64) -> OrderedCollectionPageBuilder<Url, i64, O> {
        OrderedCollectionPageBuilder {
            id: self.id,
            total_items: value,
            ordered_items: self.ordered_items,
            part_of: self.part_of,
        }
    }

    pub fn ordered_items(
        self,
        value: Vec<OrderedItem>,
    ) -> OrderedCollectionPageBuilder<Url, T, Vec<OrderedItem>> {
        OrderedCollectionPageBuilder {
            id: self.id,
            total_items: self.total_items,
            ordered_items: value,
            part_of: self.part_of,
        }
    }

    pub fn part_of(self, value: impl Into<String>) -> Self {
        OrderedCollectionPageBuilder {
            part_of: Some(Url::try_from(value.into()).unwrap()),
            ..self
        }
    }
}

impl OrderedCollectionPageBuilder<Url, i64, Vec<OrderedItem>> {
    pub fn build(self) -> OrderedCollectionPage {
        let activity_pub = ActivityPub::new(self.id.clone(), ObjType::OrderedCollectionPage);

        OrderedCollectionPage {
            activity_pub,
            part_of: self.part_of.unwrap_or_else(|| self.id.clone()),
            total_items: self.total_items,
            ordered_items: self.ordered_items,
        }
    }
}
