use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::{ObjType, Url};

use super::OrderedItem;

#[derive(Deserialize, Serialize)]
pub struct OrderedCollectionPage {
    id: Url,
    next: Url,
    prev: Url,

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

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
}

impl OrderedCollectionPageBuilder<NoId, NoTotalItems, NoOrderedItems> {
    pub fn new() -> Self {
        OrderedCollectionPageBuilder {
            id: NoId,
            total_items: NoTotalItems,
            ordered_items: NoOrderedItems,
        }
    }
}

impl<I, T, O> OrderedCollectionPageBuilder<I, T, O> {
    pub fn id(self, value: String) -> OrderedCollectionPageBuilder<Url, T, O> {
        OrderedCollectionPageBuilder {
            id: Url::try_from(value).unwrap(),
            total_items: self.total_items,
            ordered_items: self.ordered_items,
        }
    }

    pub fn total_items(self, value: i64) -> OrderedCollectionPageBuilder<I, i64, O> {
        OrderedCollectionPageBuilder {
            id: self.id,
            total_items: value,
            ordered_items: self.ordered_items,
        }
    }

    pub fn ordered_items(
        self,
        value: Vec<OrderedItem>,
    ) -> OrderedCollectionPageBuilder<I, T, Vec<OrderedItem>> {
        OrderedCollectionPageBuilder {
            id: self.id,
            total_items: self.total_items,
            ordered_items: value,
        }
    }
}

impl OrderedCollectionPageBuilder<Url, i64, Vec<OrderedItem>> {
    pub fn build(self) -> OrderedCollectionPage {
        dotenv().ok();
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");

        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();
        let part_of = Url::try_from(format!("{base_url}/outbox/posts")).unwrap();
        let next = Url::try_from(format!("{base_url}/outbox/posts?page=true")).unwrap();
        let prev = Url::try_from(format!("{base_url}/outbox/posts?page=true")).unwrap();

        OrderedCollectionPage {
            context,
            part_of,
            next,
            prev,
            id: self.id,
            obj_type: ObjType::OrderedCollectionPage,
            total_items: self.total_items,
            ordered_items: self.ordered_items,
        }
    }
}
