use std::time::SystemTime;

use chrono::{DateTime, Utc};
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::posts;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_public: bool,
    pub slug: String,
    pub created_at: SystemTime,
    pub author: String,
    pub ap_id: Uuid,
    pub updated_at: SystemTime,
    pub visibility: String,
}

#[derive(Serialize)]
pub struct GetPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_public: bool,
    pub slug: String,
    pub created_at: String,
    pub author: String,
    pub ap_id: String,
    pub updated_at: String,
    pub visibility: String,
}

impl From<&Post> for GetPost {
    fn from(value: &Post) -> Self {
        let date: DateTime<Utc> = value.created_at.into();
        let updated: DateTime<Utc> = value.updated_at.into();
        GetPost {
            id: value.id,
            title: value.title.clone(),
            body: value.body.clone(),
            is_public: value.is_public,
            slug: value.slug.clone(),
            created_at: format!("{}", date.format("%+")),
            author: value.author.clone(),
            ap_id: value.ap_id.to_string(),
            updated_at: format!("{}", updated.format("%+")),
            visibility: value.visibility.clone(),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub is_public: bool,
    pub slug: String,
    pub author: String,
    pub visibility: String,
}
