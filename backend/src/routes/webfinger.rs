use std::env;

use actix_web::{dev::Path, get, http::Uri, web::Query, HttpResponse};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::routes::users::has_user;

#[derive(Deserialize, Serialize)]
struct WebfingerLink {
    rel: String,
    href: String,

    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Deserialize, Serialize)]
struct Webfinger {
    subject: String,
    links: Vec<WebfingerLink>,
}

#[derive(Deserialize)]
struct WebfingerQuery {
    resource: String,
}

#[get(".well-known/webfinger")]
pub async fn webfinger(query: Query<WebfingerQuery>) -> HttpResponse {
    dotenv().ok();
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");
    let base_url_name = env::var("BASE_URL_NAME").expect("BASE_URL_NAME must be set!");

    let subject = format!("acct:caio@{base_url_name}");

    if query.resource != subject {
        return HttpResponse::BadRequest().finish();
    }

    let link = WebfingerLink {
        rel: "self".into(),
        link_type: "application/activity+json".into(),
        href: format!("{base_url}/users/caio"),
    };

    let response = Webfinger {
        subject,
        links: vec![link],
    };

    HttpResponse::Ok()
        .content_type(
            "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
        )
        .json(response)
}
