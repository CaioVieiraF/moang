use std::env;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::models::Post;

use super::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct ContentObject {
    id: Url,
    summary: Option<String>,
    published: String,
    url: Url,
    attributed_to: Url,
    to: Vec<Url>,
    content: String,
    name: String,

    #[serde(rename = "inReplyTo")]
    in_reply_to: Option<String>,

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,
}

impl From<&Post> for ContentObject {
    fn from(value: &Post) -> Self {
        dotenv().ok();
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");

        let attributed_to = Url::try_from(format!("{base_url}/users/caio")).unwrap();
        let date: DateTime<Utc> = value.created_at.into();
        let url = Url::try_from(format!("{base_url}/posts/{}", value.id)).unwrap();
        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();
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
            to,
            name: value.title.clone(),
            content: value.body.clone(),
            in_reply_to: None,
            context,
            obj_type: ObjType::Note,
        }
    }
}
