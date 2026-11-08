use std::time::SystemTime;

use actix_web::{get, HttpResponse};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::{config::AppConfig, establish_connection, models::Post};

const FEED_LIMIT: i64 = 20;
const DESCRIPTION_LIMIT: usize = 400;

/// RSS 2.0 feed with the public posts. The header and `index.html` already
/// advertise `/feed.xml`; this is the endpoint behind that link.
#[get("/feed.xml")]
pub async fn feed() -> HttpResponse {
    use crate::schema::posts::dsl;

    let config = AppConfig::get();
    let connection = &mut establish_connection();
    let query = dsl::posts
        .filter(dsl::is_public.eq(true))
        .order(dsl::created_at.desc())
        .limit(FEED_LIMIT)
        .select(Post::as_select())
        .load::<Post>(connection);

    let posts = match query {
        Ok(posts) => posts,
        Err(error) => {
            log::error!("could not build feed: {error}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let items = posts
        .iter()
        .map(|post| FeedItem {
            title: post.title.clone(),
            link: config.frontend_post_url(post.id),
            pub_date: rss_date(post.created_at),
            description: description(&post.body),
        })
        .collect::<Vec<FeedItem>>();

    let feed_url = format!("{}/feed.xml", config.base_url());
    let xml = render_feed(&items, config.frontend_url(), &feed_url);

    HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(xml)
}

struct FeedItem {
    title: String,
    link: String,
    pub_date: String,
    description: String,
}

fn render_feed(items: &[FeedItem], site_url: &str, feed_url: &str) -> String {
    let last_build_date = items
        .first()
        .map(|item| item.pub_date.as_str())
        .unwrap_or_else(|| "");

    let mut entries = String::new();
    for item in items {
        let title = xml_escape(&item.title);
        let link = xml_escape(&item.link);
        let description = xml_escape(&item.description);

        entries.push_str(&format!(
            "    <item>\n      <title>{title}</title>\n      <link>{link}</link>\n      <guid isPermaLink=\"true\">{link}</guid>\n      <pubDate>{}</pubDate>\n      <description>{description}</description>\n    </item>\n",
            xml_escape(&item.pub_date)
        ));
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>Moang Blog</title>
    <link>{site}</link>
    <description>Publicações do Moang Blog</description>
    <language>pt-br</language>
    <lastBuildDate>{last_build}</lastBuildDate>
    <atom:link href="{feed}" rel="self" type="application/rss+xml" />
{entries}  </channel>
</rss>
"#,
        site = xml_escape(site_url),
        feed = xml_escape(feed_url),
        last_build = xml_escape(last_build_date),
        entries = entries,
    )
}

fn rss_date(value: SystemTime) -> String {
    let date: DateTime<Utc> = value.into();
    date.format("%a, %d %b %Y %H:%M:%S %z").to_string()
}

/// Markdown is turned into plain text so feed readers do not show raw syntax.
fn description(body: &str) -> String {
    let mut text = String::new();
    for event in Parser::new(body) {
        match event {
            Event::Text(value) | Event::Code(value) => text.push_str(&value),
            Event::SoftBreak | Event::HardBreak | Event::Rule => text.push(' '),
            Event::Start(Tag::Paragraph | Tag::Heading { .. } | Tag::Item)
            | Event::End(TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::Item) => {
                text.push(' ')
            }
            _ => {}
        }
    }

    let normalized = text.split_whitespace().collect::<Vec<&str>>().join(" ");
    let mut short: String = normalized.chars().take(DESCRIPTION_LIMIT).collect();
    if normalized.chars().count() > DESCRIPTION_LIMIT {
        if let Some(index) = short.rfind(char::is_whitespace) {
            short.truncate(index);
        }
        short.push('…');
    }
    short
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::{description, render_feed, rss_date, xml_escape, FeedItem};
    use std::time::SystemTime;

    fn item(title: &str) -> FeedItem {
        FeedItem {
            title: title.into(),
            link: "https://moang.com.br/post/1".into(),
            pub_date: rss_date(SystemTime::UNIX_EPOCH),
            description: "Olá & <mundo>".into(),
        }
    }

    #[test]
    fn escapes_all_xml_special_characters() {
        assert_eq!(
            xml_escape("a & b < c > d \" e ' f"),
            "a &amp; b &lt; c &gt; d &quot; e &apos; f"
        );
    }

    #[test]
    fn dates_use_rfc_2822() {
        assert_eq!(
            rss_date(SystemTime::UNIX_EPOCH),
            "Thu, 01 Jan 1970 00:00:00 +0000"
        );
    }

    #[test]
    fn descriptions_are_plain_text() {
        assert_eq!(
            description("# Título\n\nUm **texto** com `código` e [link](https://x)."),
            "Título Um texto com código e link."
        );
        assert!(description(&"a".repeat(500)).ends_with('…'));
    }

    #[test]
    fn feed_is_a_valid_rss_document() {
        let xml = render_feed(
            &[item("Post & <teste>")],
            "https://moang.com.br",
            "https://moang.com.br/feed.xml",
        );

        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<rss version=\"2.0\""));
        assert!(xml.contains("<atom:link href=\"https://moang.com.br/feed.xml\""));
        assert!(xml.contains("<pubDate>Thu, 01 Jan 1970 00:00:00 +0000</pubDate>"));
        assert!(xml.contains("<title>Post &amp; &lt;teste&gt;</title>"));
        assert!(xml.contains("<description>Olá &amp; &lt;mundo&gt;</description>"));
        assert!(xml.contains("<guid isPermaLink=\"true\">https://moang.com.br/post/1</guid>"));
        assert!(xml.trim_end().ends_with("</rss>"));
    }
}
