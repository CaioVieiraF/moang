use std::env;

use actix_web::{get, web::Path, HttpResponse};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::models::User;

use super::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Actor<U = Url> {
    id: U,
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

    #[serde(rename = "@context")]
    context: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

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

        let context = Url::try_from("https://www.w3.org/ns/activitystreams".to_string()).unwrap();
        let id = Url::try_from(format!("{base_url}/users/{name}")).unwrap();
        let inbox = Url::try_from(format!("{base_url}/inbox")).unwrap();
        let outbox = Url::try_from(format!("{base_url}/outbox")).unwrap();
        let liked = Url::try_from(format!("{base_url}/liked")).unwrap();
        let following = Url::try_from(format!("{base_url}/following")).unwrap();
        let followers = Url::try_from(format!("{base_url}/followers")).unwrap();

        let url = Url::try_from("https://moang.com.br".to_string()).unwrap();

        Actor {
            prefered_username: name.clone(),
            obj_type: ObjType::Person,
            discoverable: true,
            indexable: true,
            context,
            id,
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

#[get("/{user}")]
pub async fn get_actor(user: Path<String>) -> HttpResponse {
    let user = user.into_inner();
    let actor = Actor::from_user(user);
    HttpResponse::Ok().json(actor)
}
