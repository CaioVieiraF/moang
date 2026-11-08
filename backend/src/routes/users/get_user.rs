use actix_web::{
    get,
    http::header,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};

use crate::{
    config::AppConfig,
    routes::{
        activity_pub::{actor::Actor, keys::get_or_create_key},
        users::has_user,
    },
};

/// Serves the actor as AS2 to fediverse software and redirects browsers to the
/// human-readable profile page, the same way Mastodon does.
#[get("")]
pub async fn get_user(req: HttpRequest, path: Path<String>) -> HttpResponse {
    if prefers_html(&req) {
        return HttpResponse::Found()
            .insert_header((header::LOCATION, AppConfig::get().profile_url()))
            .finish();
    }

    let user_name = path.into_inner().to_lowercase();
    let Some(retrieved_user) = has_user(user_name) else {
        return HttpResponse::NotFound().finish();
    };

    let key = match get_or_create_key(&retrieved_user.id) {
        Ok(key) => key,
        Err(error) => return HttpResponse::InternalServerError().body(error),
    };

    match Actor::from_user(&retrieved_user, &key) {
        Ok(actor) => HttpResponse::Ok()
            .content_type("application/activity+json")
            .json(Json(actor)),
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

fn prefers_html(req: &HttpRequest) -> bool {
    let Some(accept) = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };

    accept.contains("text/html")
        && !accept.contains("application/activity+json")
        && !accept.contains("application/ld+json")
}

#[cfg(test)]
mod tests {
    use super::prefers_html;
    use actix_web::{http::header, test::TestRequest};

    fn request(accept: Option<&str>) -> actix_web::HttpRequest {
        let mut request = TestRequest::default();
        if let Some(accept) = accept {
            request = request.insert_header((header::ACCEPT, accept));
        }
        request.to_http_request()
    }

    #[test]
    fn browsers_are_redirected_to_the_profile() {
        assert!(prefers_html(&request(Some(
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
        ))));
    }

    #[test]
    fn activity_pub_clients_keep_receiving_the_actor() {
        assert!(!prefers_html(&request(Some("application/activity+json"))));
        assert!(!prefers_html(&request(Some(
            "text/html, application/activity+json;q=0.9"
        ))));
        assert!(!prefers_html(&request(Some("*/*"))));
        assert!(!prefers_html(&request(None)));
    }
}
