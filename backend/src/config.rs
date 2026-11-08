use std::{env, sync::OnceLock};

use url::Url;

/// Configuration shared by the whole application.
///
/// ActivityPub is very sensitive to URLs: the actor id, the WebFinger subject and
/// the object ids must all use the same public domain. That is why URLs are
/// always built from this config instead of from the incoming `Host` header.
#[derive(Clone, Debug)]
pub struct AppConfig {
    base_url: Url,
    frontend_url: Url,
    domain: String,
    excerpt_length: usize,
    max_delivery_attempts: i32,
    worker_enabled: bool,
    user_agent: String,
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn get() -> &'static AppConfig {
        CONFIG.get_or_init(Self::from_env)
    }

    pub fn from_env() -> Self {
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set!");
        let base_url = base_url.trim_end_matches('/').to_string();
        let parsed_base_url = Url::parse(&base_url).expect("BASE_URL must be a valid URL!");

        if parsed_base_url.scheme() != "https" && !is_local_host(&parsed_base_url) {
            panic!("BASE_URL must use https outside local development!");
        }

        let frontend_url = env::var("FRONTEND_URL")
            .unwrap_or_else(|_| base_url.clone())
            .trim_end_matches('/')
            .to_string();
        let frontend_url = Url::parse(&frontend_url).expect("FRONTEND_URL must be a valid URL!");

        let domain = env::var("BASE_URL_NAME")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| host_with_port(&parsed_base_url));

        let excerpt_length = env::var("AP_EXCERPT_LENGTH")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(600);

        let max_delivery_attempts = env::var("AP_MAX_DELIVERY_ATTEMPTS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(8);

        let worker_enabled = env::var("AP_WORKER_ENABLED")
            .map(|value| value != "false" && value != "0")
            .unwrap_or(true);

        let user_agent = format!("moang-blog/{} (+{})", env!("CARGO_PKG_VERSION"), base_url);

        Self {
            base_url: parsed_base_url,
            frontend_url,
            domain,
            excerpt_length,
            max_delivery_attempts,
            worker_enabled,
            user_agent,
        }
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str().trim_end_matches('/')
    }

    pub fn frontend_url(&self) -> &str {
        self.frontend_url.as_str().trim_end_matches('/')
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn excerpt_length(&self) -> usize {
        self.excerpt_length
    }

    pub fn max_delivery_attempts(&self) -> i32 {
        self.max_delivery_attempts
    }

    pub fn worker_enabled(&self) -> bool {
        self.worker_enabled
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn actor_url(&self, username: &str) -> String {
        format!("{}/users/{username}", self.base_url())
    }

    pub fn actor_key_id(&self, username: &str) -> String {
        format!("{}#main-key", self.actor_url(username))
    }

    pub fn inbox_url(&self, username: &str) -> String {
        format!("{}/inbox", self.actor_url(username))
    }

    pub fn outbox_url(&self, username: &str) -> String {
        format!("{}/outbox", self.actor_url(username))
    }

    pub fn followers_url(&self, username: &str) -> String {
        format!("{}/followers", self.actor_url(username))
    }

    pub fn following_url(&self, username: &str) -> String {
        format!("{}/following", self.actor_url(username))
    }

    pub fn shared_inbox_url(&self) -> String {
        format!("{}/inbox", self.base_url())
    }

    pub fn object_url(&self, username: &str, ap_id: &uuid::Uuid) -> String {
        format!("{}/posts/{ap_id}", self.actor_url(username))
    }

    pub fn activity_url(&self, username: &str, activity_id: &uuid::Uuid) -> String {
        format!("{}/activities/{activity_id}", self.actor_url(username))
    }

    pub fn frontend_post_url(&self, post_id: i32) -> String {
        format!("{}/post/{post_id}", self.frontend_url())
    }

    pub fn profile_url(&self) -> String {
        format!("{}/about", self.frontend_url())
    }

    pub fn nodeinfo_url(&self) -> String {
        format!("{}/nodeinfo/2.1", self.base_url())
    }
}

fn host_with_port(url: &Url) -> String {
    let host = url.host_str().unwrap_or_default();
    match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_string(),
    }
}

fn is_local_host(url: &Url) -> bool {
    matches!(
        url.host_str(),
        Some("localhost") | Some("127.0.0.1") | Some("0.0.0.0") | Some("::1")
    )
}
