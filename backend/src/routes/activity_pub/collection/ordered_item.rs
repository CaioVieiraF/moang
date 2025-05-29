use std::env;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::{
    models::Post,
    routes::activity_pub::{content_object::ContentObject, ObjType, Url},
};

#[derive(Deserialize, Serialize)]
pub struct OrderedItem {
    id: Url,
    actor: Url,
    published: String,
    to: Vec<Url>,
    cc: Vec<Url>,
    object: ContentObject,
    name: String,

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,
}

pub struct NoId;
pub struct NoPublished;
pub struct NoObject;
pub struct NoName;

pub struct OrderedItemBuilder<I, P, O, N> {
    id: I,
    published: P,
    object: O,
    name: N,
}

impl OrderedItemBuilder<NoId, NoPublished, NoObject, NoName> {
    pub fn new() -> Self {
        OrderedItemBuilder {
            id: NoId,
            published: NoPublished,
            object: NoObject,
            name: NoName,
        }
    }
}

impl<I, P, O, N> OrderedItemBuilder<I, P, O, N> {
    pub fn id(self, value: i32) -> OrderedItemBuilder<Url, P, O, N> {
        dotenv().ok();
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");

        OrderedItemBuilder {
            id: Url::try_from(format!("{base_url}/posts/{value}")).unwrap(),
            published: self.published,
            object: self.object,
            name: self.name,
        }
    }

    pub fn object(self, value: ContentObject) -> OrderedItemBuilder<I, P, ContentObject, N> {
        OrderedItemBuilder {
            id: self.id,
            published: self.published,
            object: value,
            name: self.name,
        }
    }

    pub fn name(self, value: String) -> OrderedItemBuilder<I, P, O, String> {
        OrderedItemBuilder {
            id: self.id,
            published: self.published,
            object: self.object,
            name: value,
        }
    }

    pub fn published(self, value: String) -> OrderedItemBuilder<I, String, O, N> {
        OrderedItemBuilder {
            id: self.id,
            published: value,
            object: self.object,
            name: self.name,
        }
    }
}

impl OrderedItemBuilder<Url, String, ContentObject, String> {
    pub fn build(self) -> OrderedItem {
        dotenv().ok();
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");

        let actor = Url::try_from(format!("{base_url}/users/caio")).unwrap();
        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();
        let to = vec!["https://www.w3.org/ns/activitystreams#Public"
            .to_string()
            .try_into()
            .unwrap()];
        let cc = vec![format!("{base_url}/followers").try_into().unwrap()];

        OrderedItem {
            id: self.id,
            actor,
            published: self.published,
            to,
            cc,
            name: self.name,
            context,
            obj_type: ObjType::Create,
            object: self.object,
        }
    }
}

impl From<&Post> for OrderedItem {
    fn from(value: &Post) -> Self {
        let date: DateTime<Utc> = value.created_at.into();
        let post = OrderedItemBuilder::new()
            .id(value.id)
            .published(format!("{}", date.format("%+")))
            .object(ContentObject::from(value))
            .name(value.title.clone())
            .build();
        post
    }
}
