use std::env;

use actix_web::{get, web::Path, HttpResponse};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::models::User;

use super::{ActivityPub, ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Actor<U = Url> {
    #[serde(flatten)]
    activity_pub: ActivityPub,

    inbox: U,
    outbox: U,
    following: U,
    followers: U,
    liked: U,
    url: U,
    summary: String,
    discoverable: bool,
    indexable: bool,
    name: String,

    #[serde(rename = "preferredUsername")]
    prefered_username: String,
}

#[derive(Deserialize, Serialize, Default)]
struct PubKey<'a> {
    id: &'a str,
    owner: &'a str,
    #[serde(rename = "publicKeyPem")]
    public_key_pem: &'a str,
}

impl Actor {
    fn from_user(name: String) -> Actor {
        dotenv().ok();

        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");
        let summary = env::var("USER_SUMMARY").unwrap_or_default();

        let actor = format!("{base_url}/users/{name}");
        let activity_pub = ActivityPub::new(actor.clone().try_into().unwrap(), ObjType::Person);
        let inbox = Url::try_from(format!("{actor}/inbox")).unwrap();
        let outbox = Url::try_from(format!("{actor}/outbox")).unwrap();
        let liked = Url::try_from(format!("{actor}/liked")).unwrap();
        let following = Url::try_from(format!("{actor}/following")).unwrap();
        let followers = Url::try_from(format!("{actor}/followers")).unwrap();

        let url = Url::try_from("https://moang.com.br".to_string()).unwrap();

        Actor {
            activity_pub,
            prefered_username: name.clone(),
            discoverable: true,
            indexable: true,
            inbox,
            outbox,
            liked,
            name,
            following,
            followers,
            url,
            summary,
        }
    }
}

impl From<&User> for Actor {
    fn from(value: &User) -> Self {
        Actor::from_user(value.name.clone())
    }
}
