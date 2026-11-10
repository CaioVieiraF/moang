use actix_session::Session;
use actix_web::{post, HttpResponse};

/// Clears the session created by `POST /login`.
#[post("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    session.purge();
    HttpResponse::NoContent().finish()
}
