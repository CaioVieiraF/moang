use actix_session::Session;
use actix_web::{get, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{current_user, user_is_loged_in};

#[derive(Serialize)]
pub struct CurrentUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub display_name: String,
    pub summary: String,
    pub avatar_url: String,
    pub header_url: String,
    pub created_at: String,
    pub is_locked: bool,
    pub is_discoverable: bool,
}

/// Data of the logged-in user, so clients do not have to infer it from login.
#[get("/me")]
pub async fn me(session: Session) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let Some(user) = current_user(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let created_at: DateTime<Utc> = user.created_at.into();

    HttpResponse::Ok().json(CurrentUser {
        id: user.id,
        name: user.name,
        email: user.email,
        display_name: user.display_name,
        summary: user.summary,
        avatar_url: user.avatar_url,
        header_url: user.header_url,
        created_at: format!("{}", created_at.format("%+")),
        is_locked: user.is_locked,
        is_discoverable: user.is_discoverable,
    })
}
