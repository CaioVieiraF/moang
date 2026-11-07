use actix_web::{get, http::header, web::Query, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{config::AppConfig, routes::users::has_user};

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

    #[serde(default)]
    rel: Option<String>,
}

#[get("/.well-known/webfinger")]
pub async fn webfinger(query: Query<WebfingerQuery>) -> HttpResponse {
    let config = AppConfig::get();

    let Some(resource) = query.resource.strip_prefix("acct:") else {
        return HttpResponse::BadRequest().finish();
    };

    let Some((username, domain)) = resource.split_once('@') else {
        return HttpResponse::BadRequest().finish();
    };

    if !domain.eq_ignore_ascii_case(config.domain()) {
        return HttpResponse::NotFound().finish();
    }

    let username = username.to_lowercase();
    if has_user(username.clone()).is_none() {
        return HttpResponse::NotFound().finish();
    }

    let mut links = vec![
        WebfingerLink {
            rel: "self".into(),
            link_type: "application/activity+json".into(),
            href: config.actor_url(&username),
        },
        WebfingerLink {
            rel: "http://webfinger.net/rel/profile-page".into(),
            link_type: "text/html".into(),
            href: config.profile_url(),
        },
    ];

    if let Some(rel) = query.rel.as_deref() {
        links.retain(|link| link.rel == rel);
    }

    let response = Webfinger {
        subject: format!("acct:{username}@{}", config.domain()),
        links,
    };

    HttpResponse::Ok()
        .content_type("application/jrd+json")
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((header::CACHE_CONTROL, "public, max-age=300"))
        .json(response)
}
