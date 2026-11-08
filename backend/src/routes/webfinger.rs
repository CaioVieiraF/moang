use actix_web::{get, http::header, web::Query, HttpResponse};
use serde::{Deserialize, Serialize};
use url::Url;

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

/// WebFinger is how other servers turn `@user@domain` into the actor URL.
/// Besides `acct:` URIs, some implementations query with the actor URL itself
/// or with a bare `user@domain`, so all three shapes are accepted.
#[get("/.well-known/webfinger")]
pub async fn webfinger(query: Query<WebfingerQuery>) -> HttpResponse {
    let config = AppConfig::get();

    let Some(username) = username_from_resource(&query.resource, config.domain()) else {
        return HttpResponse::NotFound().finish();
    };

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

/// XRD host-meta, used by older OStatus/GNU Social style software to find the
/// WebFinger endpoint before looking up an account.
#[get("/.well-known/host-meta")]
pub async fn host_meta() -> HttpResponse {
    let template = webfinger_template();

    HttpResponse::Ok()
        .content_type("application/xrd+xml; charset=utf-8")
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .body(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <XRD xmlns=\"http://docs.oasis-open.org/ns/xri/xrd-1.0\">\n  \
             <Link rel=\"lrdd\" type=\"application/jrd+json\" template=\"{template}\" />\n\
             </XRD>\n"
        ))
}

/// JSON (JRD) variant of host-meta, used by newer clients.
#[get("/.well-known/host-meta.json")]
pub async fn host_meta_json() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/jrd+json")
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .json(serde_json::json!({
            "links": [{
                "rel": "lrdd",
                "type": "application/jrd+json",
                "template": webfinger_template(),
            }]
        }))
}

fn webfinger_template() -> String {
    format!(
        "{}/.well-known/webfinger?resource={{uri}}",
        AppConfig::get().base_url()
    )
}

fn username_from_resource(resource: &str, domain: &str) -> Option<String> {
    let resource = resource.trim();
    let resource = if resource.len() >= 5 && resource[..5].eq_ignore_ascii_case("acct:") {
        &resource[5..]
    } else {
        resource
    };

    let username = if resource.starts_with("http://") || resource.starts_with("https://") {
        let url = Url::parse(resource).ok()?;
        if !url.host_str()?.eq_ignore_ascii_case(domain) {
            return None;
        }

        let segments = url.path_segments()?.collect::<Vec<&str>>();
        let position = segments.iter().position(|segment| *segment == "users")?;
        segments.get(position + 1)?.to_string()
    } else {
        let (username, resource_domain) = resource.split_once('@')?;
        if !resource_domain.eq_ignore_ascii_case(domain) {
            return None;
        }
        username.to_string()
    };

    let username = username.to_lowercase();
    if username.is_empty() {
        return None;
    }

    Some(username)
}

#[cfg(test)]
mod tests {
    use super::username_from_resource;

    #[test]
    fn accepts_acct_uris_case_insensitively() {
        assert_eq!(
            username_from_resource("acct:caio@moang.com.br", "moang.com.br"),
            Some("caio".to_string())
        );
        assert_eq!(
            username_from_resource("ACCT:Caio@MOANG.COM.BR", "moang.com.br"),
            Some("caio".to_string())
        );
    }

    #[test]
    fn accepts_bare_accounts_and_actor_urls() {
        assert_eq!(
            username_from_resource("caio@moang.com.br", "moang.com.br"),
            Some("caio".to_string())
        );
        assert_eq!(
            username_from_resource("https://moang.com.br/users/caio", "moang.com.br"),
            Some("caio".to_string())
        );
        assert_eq!(
            username_from_resource("https://moang.com.br/users/caio/", "moang.com.br"),
            Some("caio".to_string())
        );
    }

    #[test]
    fn rejects_other_domains_and_unknown_urls() {
        assert_eq!(
            username_from_resource("acct:caio@outro.com", "moang.com.br"),
            None
        );
        assert_eq!(
            username_from_resource("https://outro.com/users/caio", "moang.com.br"),
            None
        );
        assert_eq!(
            username_from_resource("https://moang.com.br/about", "moang.com.br"),
            None
        );
        assert_eq!(username_from_resource("acct:@moang.com.br", "moang.com.br"), None);
    }
}
