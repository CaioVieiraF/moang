use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    establish_connection,
    models::Post,
    schema::remote_posts,
};

use super::{collection::Collection, Context, ObjType, Url, PUBLIC_COLLECTION};

#[derive(Deserialize, Serialize)]
pub struct ContentObject {
    #[serde(rename = "@context")]
    context: Context,

    id: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    name: String,
    summary: Option<String>,
    content: String,
    source: Source,
    published: String,
    updated: String,
    url: Url,

    #[serde(rename = "attributedTo")]
    attributed_to: Url,

    to: Vec<Url>,
    cc: Vec<Url>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachment: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tag: Vec<serde_json::Value>,

    #[serde(rename = "inReplyTo", skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<String>,

    replies: Collection,
}

#[derive(Deserialize, Serialize)]
struct Source {
    content: String,

    #[serde(rename = "mediaType")]
    media_type: String,
}

impl ContentObject {
    pub fn to(&self) -> &[Url] {
        &self.to
    }

    pub fn cc(&self) -> &[Url] {
        &self.cc
    }

    pub fn from_post(value: &Post, username: &str) -> Result<ContentObject, String> {
        let config = AppConfig::get();
        let object_url = config.object_url(username, &value.ap_id);
        let object_url = Url::try_from(object_url.clone())?;
        let followers_url = config.followers_url(username);

        let (to, cc) = audience(&value.visibility, &followers_url)?;
        let published: DateTime<Utc> = value.created_at.into();
        let updated: DateTime<Utc> = value.updated_at.into();

        let replies_count = connection_count(&object_url)?;
        let replies = Collection::with_total(
            Url::try_from(format!("{object_url}/replies"))?,
            replies_count,
        );

        Ok(ContentObject {
            context: Context::activity_streams(),
            id: object_url,
            obj_type: ObjType::Article,
            name: value.title.clone(),
            summary: None,
            content: excerpt(&value.body, config.excerpt_length(), &config.frontend_post_url(value.id)),
            source: Source {
                content: value.body.clone(),
                media_type: "text/markdown".into(),
            },
            published: format!("{}", published.format("%+")),
            updated: format!("{}", updated.format("%+")),
            url: Url::try_from(config.frontend_post_url(value.id))?,
            attributed_to: Url::try_from(config.actor_url(username))?,
            to,
            cc,
            attachment: Vec::new(),
            tag: Vec::new(),
            in_reply_to: None,
            replies,
        })
    }
}

fn audience(visibility: &str, followers_url: &str) -> Result<(Vec<Url>, Vec<Url>), String> {
    let public = Url::try_from(PUBLIC_COLLECTION)?;
    let followers = Url::try_from(followers_url)?;

    let (to, cc) = match visibility {
        "public" => (vec![public], vec![followers]),
        "unlisted" => (vec![followers], vec![public]),
        "followers" => (vec![followers], Vec::new()),
        _ => (Vec::new(), Vec::new()),
    };

    Ok((to, cc))
}

fn connection_count(object_url: &Url) -> Result<i32, String> {
    let connection = &mut establish_connection();
    let count = remote_posts::table
        .filter(remote_posts::in_reply_to.eq(object_url.as_str()))
        .count()
        .get_result::<i64>(connection)
        .map_err(|error| error.to_string())?;

    Ok(count as i32)
}

fn excerpt(body: &str, length: usize, full_url: &str) -> String {
    let truncated = truncate(body, length);
    let mut html_output = String::new();
    let parser = Parser::new(&truncated);
    html::push_html(&mut html_output, parser);

    html_output.push_str(&format!(
        "<p><a href=\"{full_url}\">Ler o post completo</a></p>"
    ));

    ammonia::clean(&html_output)
}

fn truncate(value: &str, length: usize) -> String {
    if value.chars().count() <= length {
        return value.to_string();
    }

    let mut result: String = value.chars().take(length).collect();
    if let Some(index) = result.rfind(char::is_whitespace) {
        result.truncate(index);
    }
    result.push_str("...");
    result
}

#[cfg(test)]
mod tests {
    use super::{audience, truncate, PUBLIC_COLLECTION};

    #[test]
    fn public_posts_are_addressed_to_the_public_collection() {
        let (to, cc) = audience("public", "https://example.com/followers").unwrap();

        assert_eq!(to[0].as_str(), PUBLIC_COLLECTION);
        assert_eq!(cc[0].as_str(), "https://example.com/followers");
    }

    #[test]
    fn unlisted_posts_put_the_public_collection_in_cc() {
        let (to, cc) = audience("unlisted", "https://example.com/followers").unwrap();

        assert_eq!(to[0].as_str(), "https://example.com/followers");
        assert_eq!(cc[0].as_str(), PUBLIC_COLLECTION);
    }

    #[test]
    fn private_posts_have_no_audience() {
        let (to, cc) = audience("private", "https://example.com/followers").unwrap();

        assert!(to.is_empty());
        assert!(cc.is_empty());
    }

    #[test]
    fn truncation_keeps_the_text_short() {
        let text = "uma frase um pouco maior do que o limite";

        assert!(truncate(text, 10).ends_with("..."));
        assert_eq!(truncate("curto", 10), "curto");
    }
}
