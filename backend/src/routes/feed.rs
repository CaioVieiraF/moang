use std::time::SystemTime;

use actix_web::{get, http::header, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use sha2::{Digest, Sha256};

use crate::{
    config::AppConfig,
    establish_connection,
    models::Post,
    routes::activity_pub::content_object::markdown_to_html,
};

const FEED_LIMIT: i64 = 20;
const DESCRIPTION_LIMIT: usize = 400;
const CACHE_CONTROL: &str = "public, max-age=300";

/// RSS 2.0 feed with the public posts. The header and `index.html` already
/// advertise `/feed.xml`; this is the endpoint behind that link.
#[get("/feed.xml")]
pub async fn feed(req: HttpRequest) -> HttpResponse {
    use crate::schema::posts::dsl;

    let config = AppConfig::get();
    let connection = &mut establish_connection();
    let query = dsl::posts
        .filter(dsl::is_public.eq(true))
        .order((dsl::created_at.desc(), dsl::id.desc()))
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
            content: markdown_to_html(&post.body),
        })
        .collect::<Vec<FeedItem>>();

    let feed_url = format!("{}/feed.xml", config.base_url());
    let xml = render_feed(&items, config.frontend_url(), &feed_url);
    let etag = entity_tag(&xml);

    if request_matches_etag(&req, &etag) {
        return HttpResponse::NotModified()
            .insert_header((header::ETAG, etag))
            .insert_header((header::CACHE_CONTROL, CACHE_CONTROL))
            .finish();
    }

    HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .insert_header((header::ETAG, etag))
        .insert_header((header::CACHE_CONTROL, CACHE_CONTROL))
        .body(xml)
}

struct FeedItem {
    title: String,
    link: String,
    pub_date: String,
    description: String,
    content: String,
}

fn render_feed(items: &[FeedItem], site_url: &str, feed_url: &str) -> String {
    // An empty `<lastBuildDate></lastBuildDate>` is not a valid date, so fall
    // back to the current time when there are no posts yet.
    let last_build_date = items
        .first()
        .map(|item| item.pub_date.clone())
        .unwrap_or_else(|| rss_date(SystemTime::now()));

    let mut entries = String::new();
    for item in items {
        let title = xml_escape(&item.title);
        let link = xml_escape(&item.link);
        let description = xml_escape(&item.description);
        let content = xml_escape(&item.content);

        entries.push_str(&format!(
            "    <item>\n      <title>{title}</title>\n      <link>{link}</link>\n      <guid isPermaLink=\"true\">{link}</guid>\n      <pubDate>{}</pubDate>\n      <description>{description}</description>\n      <content:encoded>{content}</content:encoded>\n    </item>\n",
            xml_escape(&item.pub_date)
        ));
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>Moang Blog</title>
    <link>{site}</link>
    <description>Publicações do Moang Blog</description>
    <language>pt-br</language>
    <generator>moang-blog</generator>
    <lastBuildDate>{last_build}</lastBuildDate>
    <atom:link href="{feed}" rel="self" type="application/rss+xml" />
{entries}  </channel>
</rss>
"#,
        site = xml_escape(site_url),
        feed = xml_escape(feed_url),
        last_build = xml_escape(&last_build_date),
        entries = entries,
    )
}

fn rss_date(value: SystemTime) -> String {
    let date: DateTime<Utc> = value.into();
    date.format("%a, %d %b %Y %H:%M:%S %z").to_string()
}

/// Markdown is turned into plain text so feed readers do not show raw syntax.
/// Raw HTML inside the Markdown is stripped to its text.
fn description(body: &str) -> String {
    let mut text = String::new();
    for event in Parser::new(body) {
        match event {
            Event::Text(value) | Event::Code(value) => text.push_str(&value),
            Event::Html(value) | Event::InlineHtml(value) => text.push_str(&strip_html(&value)),
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

fn strip_html(value: &str) -> String {
    let mut result = String::new();
    let mut inside_tag = false;
    let mut quote: Option<char> = None;

    for character in value.chars() {
        if inside_tag {
            match character {
                '"' | '\'' if quote == Some(character) => quote = None,
                '"' | '\'' if quote.is_none() => quote = Some(character),
                '>' if quote.is_none() => inside_tag = false,
                _ => {}
            }
        } else if character == '<' {
            inside_tag = true;
        } else {
            result.push(character);
        }
    }

    result
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Strong ETag for conditional GET (`If-None-Match`).
fn entity_tag(body: &str) -> String {
    let digest = Sha256::digest(body.as_bytes());
    let hex = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();

    format!("\"{hex}\"")
}

fn request_matches_etag(req: &HttpRequest, etag: &str) -> bool {
    let Some(value) = req
        .headers()
        .get(header::IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };

    value.split(',').any(|candidate| {
        let candidate = candidate.trim();
        candidate == "*" || candidate == etag || candidate.strip_prefix("W/") == Some(etag)
    })
}

#[cfg(test)]
mod tests {
    use super::{
        description, entity_tag, render_feed, rss_date, strip_html, xml_escape, FeedItem,
    };
    use std::time::SystemTime;

    fn item(title: &str) -> FeedItem {
        FeedItem {
            title: title.into(),
            link: "https://moang.com.br/post/1".into(),
            pub_date: rss_date(SystemTime::UNIX_EPOCH),
            description: "Olá & <mundo>".into(),
            content: "<p>Olá & <mundo></p>".into(),
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
    fn raw_html_inside_markdown_becomes_text() {
        assert_eq!(description("Texto <b>forte</b> aqui"), "Texto forte aqui");
        assert_eq!(strip_html("<span title=\"a>b\">oi</span>"), "oi");
        assert_eq!(strip_html("a &amp; b"), "a & b");
    }

    #[test]
    fn entity_tags_change_with_the_content() {
        let first = entity_tag("abc");

        assert_eq!(first, entity_tag("abc"));
        assert_ne!(first, entity_tag("abcd"));
        assert!(first.starts_with('"') && first.ends_with('"'));
    }

    #[test]
    fn empty_feed_still_has_a_valid_last_build_date() {
        let xml = render_feed(&[], "https://moang.com.br", "https://moang.com.br/feed.xml");
        let start = xml.find("<lastBuildDate>").unwrap() + "<lastBuildDate>".len();
        let end = xml.find("</lastBuildDate>").unwrap();
        let value = &xml[start..end];

        assert!(!value.is_empty());
        assert!(chrono::DateTime::parse_from_rfc2822(value).is_ok());
        assert!(!xml.contains("<item>"));
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
        assert!(xml.contains("xmlns:content=\"http://purl.org/rss/1.0/modules/content/\""));
        assert!(xml.contains("<generator>moang-blog</generator>"));
        assert!(xml.contains("<atom:link href=\"https://moang.com.br/feed.xml\""));
        assert!(xml.contains("<pubDate>Thu, 01 Jan 1970 00:00:00 +0000</pubDate>"));
        assert!(xml.contains("<title>Post &amp; &lt;teste&gt;</title>"));
        assert!(xml.contains("<description>Olá &amp; &lt;mundo&gt;</description>"));
        assert!(xml.contains("<content:encoded>&lt;p&gt;Olá &amp; &lt;mundo&gt;&lt;/p&gt;</content:encoded>"));
        assert!(xml.contains("<guid isPermaLink=\"true\">https://moang.com.br/post/1</guid>"));
        assert!(xml.trim_end().ends_with("</rss>"));
    }
}
