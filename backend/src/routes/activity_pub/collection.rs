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

struct NoId;
struct NoFirst;
struct NoContext;
struct NoTotalItems;

pub struct OrderedCollectionBuilder<I, F, C, T> {
    id: I,
    first: F,
    context: C,
    total_items: T,
}

impl OrderedCollectionBuilder<NoId, NoFirst, NoContext, NoTotalItems> {
    pub fn new() -> Self {
        OrderedCollectionBuilder {
            id: NoId,
            first: NoFirst,
            context: NoContext,
            total_items: NoTotalItems,
        }
    }
}

impl<I, F, C, T> OrderedCollectionBuilder<I, F, C, T> {
    pub fn id(self, value: String) -> OrderedCollectionBuilder<Url, F, C, T> {
        OrderedCollectionBuilder {
            id: Url::try_from(value).unwrap(),
            first: self.first,
            context: self.context,
            total_items: self.total_items,
        }
    }

    pub fn first(self, value: String) -> OrderedCollectionBuilder<I, Url, C, T> {
        OrderedCollectionBuilder {
            id: self.id,
            first: Url::try_from(value).unwrap(),
            context: self.context,
            total_items: self.total_items,
        }
    }

    pub fn context(self, value: String) -> OrderedCollectionBuilder<I, F, Url, T> {
        OrderedCollectionBuilder {
            id: self.id,
            first: self.first,
            context: Url::try_from(value).unwrap(),
            total_items: self.total_items,
        }
    }

    pub fn total_items(self, value: i32) -> OrderedCollectionBuilder<I, F, C, i32> {
        OrderedCollectionBuilder {
            id: self.id,
            first: self.first,
            context: self.context,
            total_items: value,
        }
    }
}

impl OrderedCollectionBuilder<Url, Url, Url, i32> {
    pub fn build(self) -> OrderedCollection {
        OrderedCollection {
            id: self.id,
            first: self.first,
            context: self.context,
            obj_type: ObjType::OrderedCollection,
            total_items: self.total_items,
        }
    }
}
