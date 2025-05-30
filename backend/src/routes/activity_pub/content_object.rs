use std::env;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::models::Post;

use super::{collection::Collection, ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct ContentObject {
    id: Url,
    summary: Option<String>,
    published: String,
    url: Url,
    to: Vec<Url>,
    content: String,
    name: String,
    attachment: Vec<String>,
    tag: Vec<String>,
    likes: Collection,
    shares: Collection,
    replies: Collection,

    #[serde(rename = "inReplyTo")]
    in_reply_to: Option<String>,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "attributedTo")]
    attributed_to: Url,
}

impl From<&Post> for ContentObject {
    fn from(value: &Post) -> Self {
        dotenv().ok();
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");

        let actor = format!("{base_url}/users/caio");
        let attributed_to = Url::try_from(actor.clone()).unwrap();
        let date: DateTime<Utc> = value.created_at.into();
        let url = Url::try_from(format!("{actor}/outbox/{}", value.id)).unwrap();
        let likes =
            Collection::new(Url::try_from(format!("{actor}/outbox/{}/likes", value.id)).unwrap());
        let shares =
            Collection::new(Url::try_from(format!("{actor}/outbox/{}/shares", value.id)).unwrap());
        let replies =
            Collection::new(Url::try_from(format!("{actor}/outbox/{}/replies", value.id)).unwrap());
        let to = vec!["https://www.w3.org/ns/activitystreams#Public"
            .to_string()
            .try_into()
            .unwrap()];

        ContentObject {
            id: url.clone(),
            summary: None,
            published: format!("{}", date.format("%+")),
            url,
            attributed_to,
            likes,
            shares,
            replies,
            to,
            name: value.title.clone(),
            attachment: Vec::new(),
            tag: Vec::new(),
            content: value.body.clone(),
            in_reply_to: None,
            obj_type: ObjType::Article,
        }
    }
}
