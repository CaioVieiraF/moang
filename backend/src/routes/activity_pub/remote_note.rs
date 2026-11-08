use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::models::RemotePost;

use super::{Context, ObjType, PUBLIC_COLLECTION};

/// A reply received from another server, re-published as a valid
/// ActivityStreams `Note`.
///
/// The `/replies` collection is advertised by our own objects, so consumers
/// that fetch it with an ActivityPub `Accept` header must find real AS2
/// objects here, not the JSON shape used by the blog frontend. Audience (`to`
/// and `cc`) is kept from the original object so remote software can decide
/// whether the note is public.
#[derive(Serialize)]
pub struct RemoteNote {
    #[serde(rename = "@context")]
    context: Context,

    id: String,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "attributedTo")]
    attributed_to: String,

    content: String,
    published: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(rename = "inReplyTo", skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<String>,

    to: Vec<String>,
    cc: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    sensitive: bool,
    attachment: Vec<Value>,
    tag: Vec<Value>,

    replies: CountCollection,
    likes: CountCollection,
    shares: CountCollection,
}

impl RemoteNote {
    pub fn new(post: &RemotePost) -> RemoteNote {
        let mut to = string_array(&post.to_addresses);
        let cc = string_array(&post.cc_addresses);

        // Older rows and servers that omit the audience are treated as public,
        // otherwise other servers would not be able to display the reply.
        if to.is_empty() && cc.is_empty() {
            to.push(PUBLIC_COLLECTION.to_string());
        }

        RemoteNote {
            context: Context::activity_streams(),
            id: post.id.clone(),
            obj_type: ObjType::Note,
            attributed_to: post.actor_id.clone(),
            content: post.content.clone(),
            published: timestamp(post.published),
            updated: post.updated_at.map(timestamp),
            url: post.url.clone(),
            in_reply_to: post.in_reply_to.clone(),
            to,
            cc,
            summary: post
                .summary
                .clone()
                .filter(|value| !value.trim().is_empty()),
            sensitive: post.sensitive,
            attachment: Vec::new(),
            tag: Vec::new(),
            replies: CountCollection::new(post.replies_count),
            likes: CountCollection::new(post.likes_count),
            shares: CountCollection::new(post.reposts_count),
        }
    }
}

/// Counts are published as plain collections, the same way our own objects
/// advertise `replies`, `likes` and `shares`.
#[derive(Serialize)]
struct CountCollection {
    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "totalItems")]
    total_items: i32,
}

impl CountCollection {
    fn new(total_items: i32) -> CountCollection {
        CountCollection {
            obj_type: ObjType::Collection,
            total_items,
        }
    }
}

fn timestamp(value: SystemTime) -> String {
    let date: DateTime<Utc> = value.into();
    format!("{}", date.format("%+"))
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{string_array, RemoteNote, PUBLIC_COLLECTION};
    use crate::models::RemotePost;
    use serde_json::{json, Value};
    use std::time::SystemTime;

    fn post(to: Value, cc: Value) -> RemotePost {
        RemotePost {
            id: "https://example.com/users/ana/notes/1".into(),
            actor_id: "https://example.com/users/ana".into(),
            content: "<p>Olá</p>".into(),
            summary: None,
            published: SystemTime::UNIX_EPOCH,
            updated_at: None,
            url: Some("https://example.com/@ana/1".into()),
            in_reply_to: Some("https://moang.com.br/users/caio/posts/abc".into()),
            sensitive: false,
            created_at: SystemTime::UNIX_EPOCH,
            likes_count: 7,
            reposts_count: 2,
            replies_count: 3,
            to_addresses: to,
            cc_addresses: cc,
        }
    }

    #[test]
    fn missing_audience_defaults_to_public() {
        let note = RemoteNote::new(&post(json!([]), json!([])));

        assert_eq!(note.to, vec![PUBLIC_COLLECTION.to_string()]);
        assert!(note.cc.is_empty());
    }

    #[test]
    fn stored_audience_is_preserved() {
        let note = RemoteNote::new(&post(
            json!(["https://example.com/users/ana"]),
            json!([PUBLIC_COLLECTION]),
        ));

        assert_eq!(note.to, vec!["https://example.com/users/ana".to_string()]);
        assert_eq!(note.cc, vec![PUBLIC_COLLECTION.to_string()]);
    }

    #[test]
    fn note_points_back_to_the_local_object() {
        let note = RemoteNote::new(&post(json!([]), json!([])));

        assert_eq!(note.attributed_to, "https://example.com/users/ana");
        assert_eq!(
            note.in_reply_to.as_deref(),
            Some("https://moang.com.br/users/caio/posts/abc")
        );
        assert_eq!(note.likes.total_items, 7);
        assert_eq!(note.shares.total_items, 2);
        assert_eq!(note.replies.total_items, 3);
        assert_eq!(note.published, "1970-01-01T00:00:00+00:00");
        assert!(note.updated.is_none());
    }

    #[test]
    fn non_string_audience_entries_are_ignored() {
        assert_eq!(
            string_array(&json!(["https://example.com/a", 42, { "id": "x" }])),
            vec!["https://example.com/a".to_string()]
        );
        assert!(string_array(&json!("https://example.com/a")).is_empty());
    }
}
