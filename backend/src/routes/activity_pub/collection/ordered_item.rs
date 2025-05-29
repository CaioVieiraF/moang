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

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,
}

pub struct NoId;
pub struct NoPublished;
pub struct NoObject;

pub struct OrderedItemBuilder<I, P, O> {
    id: I,
    published: P,
    object: O,
}

impl OrderedItemBuilder<NoId, NoPublished, NoObject> {
    pub fn new() -> Self {
        OrderedItemBuilder {
            id: NoId,
            published: NoPublished,
            object: NoObject,
        }
    }
}

impl<I, P, O> OrderedItemBuilder<I, P, O> {
    pub fn id(self, value: i32) -> OrderedItemBuilder<Url, P, O> {
        dotenv().ok();
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");

        OrderedItemBuilder {
            id: Url::try_from(format!("{base_url}/posts/{value}")).unwrap(),
            published: self.published,
            object: self.object,
        }
    }

    pub fn object(self, value: ContentObject) -> OrderedItemBuilder<I, P, ContentObject> {
        OrderedItemBuilder {
            id: self.id,
            published: self.published,
            object: value,
        }
    }

    pub fn published(self, value: String) -> OrderedItemBuilder<I, String, O> {
        OrderedItemBuilder {
            id: self.id,
            published: value,
            object: self.object,
        }
    }
}

impl OrderedItemBuilder<Url, String, ContentObject> {
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
            .build();

        post
    }
}
