use actix_session::Session;
use actix_web::{put, web::Json, web::Path, HttpResponse};
use diesel::{prelude::*, AsChangeset};
use serde::Deserialize;
use url::Url;

use crate::{
    current_user_email, establish_connection,
    models::User,
    routes::users::{has_user, outbox::publish::publish_actor_update},
    user_is_loged_in,
};

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
struct ProfileChanges {
    display_name: Option<String>,
    summary: Option<String>,
    avatar_url: Option<String>,
    header_url: Option<String>,
}

/// Update the profile fields that are published in the ActivityPub actor.
///
/// Images are plain URLs, so they can live in `frontend/public` (and be served
/// by nginx) or in any other HTTPS host.
#[put("/profile")]
pub async fn update_profile(
    session: Session,
    path: Path<String>,
    request: Json<ProfileChanges>,
) -> HttpResponse {
    use crate::schema::users;

    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let Some(session_email) = current_user_email(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username) else {
        return HttpResponse::NotFound().finish();
    };

    if session_email.to_lowercase() != user.email.to_lowercase() {
        return HttpResponse::Forbidden().finish();
    }

    let changes = match validate(request.into_inner()) {
        Ok(changes) => changes,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let connection = &mut establish_connection();
    let updated = diesel::update(users::table.find(&user.id))
        .set(&changes)
        .returning(User::as_returning())
        .get_result::<User>(connection);

    match updated {
        Ok(user) => {
            publish_actor_update(&user);
            HttpResponse::Ok().json(serde_json::json!({
                "name": user.name,
                "display_name": user.display_name,
                "summary": user.summary,
                "avatar_url": user.avatar_url,
                "header_url": user.header_url,
            }))
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

fn validate(mut changes: ProfileChanges) -> Result<ProfileChanges, String> {
    changes.display_name = normalize_text(changes.display_name, 64, "display_name")?;
    changes.summary = normalize_text(changes.summary, 2000, "summary")?;
    changes.avatar_url = normalize_url(changes.avatar_url, "avatar_url")?;
    changes.header_url = normalize_url(changes.header_url, "header_url")?;

    Ok(changes)
}

fn normalize_text(
    value: Option<String>,
    max_len: usize,
    field: &str,
) -> Result<Option<String>, String> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value.trim().to_string();

    if value.chars().count() > max_len {
        return Err(format!("{field} must have at most {max_len} characters"));
    }

    Ok(Some(value))
}

fn normalize_url(value: Option<String>, field: &str) -> Result<Option<String>, String> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value.trim().to_string();

    if value.is_empty() {
        return Ok(Some(String::new()));
    }

    if value.len() > 2048 {
        return Err(format!("{field} must have at most 2048 characters"));
    }

    let parsed = Url::parse(&value).map_err(|_| format!("{field} must be a valid URL"))?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(format!("{field} must use http or https"));
    }

    Ok(Some(value))
}
