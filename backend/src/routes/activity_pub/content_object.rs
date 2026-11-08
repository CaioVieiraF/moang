use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    establish_connection,
    models::Post,
    schema::{remote_interactions, remote_posts},
};

use super::{collection::Collection, Context, ObjType, Url, PUBLIC_COLLECTION};

#[derive(Deserialize, Serialize)]
pub struct ContentObject {
    #[serde(rename = "@context")]
    context: Context,

    id: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
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
    likes: Collection,
    shares: Collection,
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
        let obj_type = object_type(&value.kind);

        let replies_count = connection_count(&object_url)?;
        let likes_count = reaction_count(&object_url, "like")?;
        let shares_count = reaction_count(&object_url, "repost")?;
        let replies = Collection::with_total(
            Url::try_from(format!("{object_url}/replies"))?,
            replies_count,
        );
        let likes =
            Collection::with_total(Url::try_from(format!("{object_url}/likes"))?, likes_count);
        let shares =
            Collection::with_total(Url::try_from(format!("{object_url}/shares"))?, shares_count);

        // Articles federate a short excerpt plus a link to the blog. Microblog
        // posts are `Note`s, so Mastodon renders the whole content inline.
        let name = if obj_type == ObjType::Article {
            Some(value.title.clone())
        } else {
            None
        };
        let content = if obj_type == ObjType::Note {
            markdown_to_html(&value.body)
        } else {
            excerpt(
                &value.body,
                config.excerpt_length(),
                &config.frontend_post_url(value.id),
            )
        };

        Ok(ContentObject {
            context: Context::activity_streams(),
            id: object_url,
            obj_type,
            name,
            summary: None,
            content,
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
            likes,
            shares,
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

fn reaction_count(object_url: &Url, interaction_type: &str) -> Result<i32, String> {
    let connection = &mut establish_connection();
    let count = remote_interactions::table
        .filter(remote_interactions::object_id.eq(object_url.as_str()))
        .filter(remote_interactions::interaction_type.eq(interaction_type))
        .count()
        .get_result::<i64>(connection)
        .map_err(|error| error.to_string())?;

    Ok(count as i32)
}

fn object_type(kind: &str) -> ObjType {
    match kind {
        "microblog" => ObjType::Note,
        _ => ObjType::Article,
    }
}

fn markdown_to_html(body: &str) -> String {
    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(body));
    ammonia::clean(&html_output)
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
    use super::{audience, object_type, truncate, PUBLIC_COLLECTION};
    use crate::routes::activity_pub::ObjType;

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
    fn microblog_posts_use_note_objects() {
        assert_eq!(object_type("microblog"), ObjType::Note);
        assert_eq!(object_type("article"), ObjType::Article);
        assert_eq!(object_type("anything-else"), ObjType::Article);
    }

    #[test]
    fn truncation_keeps_the_text_short() {
        let text = "uma frase um pouco maior do que o limite";

        assert!(truncate(text, 10).ends_with("..."));
        assert_eq!(truncate("curto", 10), "curto");
    }
}
